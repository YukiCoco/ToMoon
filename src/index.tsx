import {
  ButtonItem,
  definePlugin,
  PanelSection,
  PanelSectionRow,
  Router,
  staticClasses,
  ToggleField,
  SidebarNavigation,
  DropdownOption,
  Navigation,
  DropdownItem,
  SliderField,
  NotchLabel,
} from "@decky/ui";
import { routerHook } from "@decky/api";
import { FC, useEffect, useState } from "react";
import { GiEgyptianBird } from "react-icons/gi";

import { Subscriptions, About, Debug } from "./pages";

import * as backend from "./backend/backend";
import axios from "axios";

import { ApiCallBackend, PyBackend, EnhancedMode } from "./backend";
import { VersionComponent } from "./components";

let enabledGlobal = false;
let enabledSkipProxy = false;
let enabledOverrideDNS = false;
let usdplReady = false;
let subs: any[];
let subs_option: any[];
let current_sub = "";
let enhanced_mode = EnhancedMode.FakeIp;
let dashboard_list: string[];
let current_dashboard = "";
let allow_remote_access = false;

const Content: FC<{}> = ({}) => {
  if (!usdplReady) {
    return <PanelSection>Init...</PanelSection>;
  }
  const [clashState, setClashState] = useState(enabledGlobal);
  backend.resolve(backend.getEnabled(), setClashState);
  const [options, setOptions] = useState<DropdownOption[]>(subs_option);
  const [optionDropdownDisabled, setOptionDropdownDisabled] =
    useState(enabledGlobal);
  const [openDashboardDisabled, setOpenDashboardDisabled] = useState(
    !enabledGlobal
  );
  const [isSelectionDisabled, setIsSelectionDisabled] = useState(false);
  const [SelectionTips, setSelectionTips] = useState("Run Clash in background");
  const [skipProxyState, setSkipProxyState] = useState(enabledSkipProxy);
  const [overrideDNSState, setOverrideDNSState] = useState(enabledOverrideDNS);
  const [currentSub, setCurrentSub] = useState<string>(current_sub);
  const [enhancedMode, setEnhancedMode] = useState<EnhancedMode>(enhanced_mode);
  const [dashboardList, setDashboardList] = useState<string[]>(dashboard_list);
  const [currentDashboard, setCurrentDashboard] =
    useState<string>(current_dashboard);
  const [allowRemoteAccess, setAllowRemoteAccess] =
    useState(allow_remote_access);

  const update_subs = () => {
    backend.resolve(backend.getSubList(), (v: String) => {
      // console.log(`getSubList: ${v}`);
      let x: Array<any> = JSON.parse(v.toString());
      let re = new RegExp("(?<=subs/).+.yaml$");
      let i = 0;
      subs = x.map((x) => {
        let name = re.exec(x.path);
        return {
          id: i++,
          name: name![0],
          url: x.url,
        };
      });
      let items = x.map((x) => {
        let name = re.exec(x.path);
        return {
          label: name![0],
          data: x.path,
        };
      });
      subs_option = items;
      setOptions(subs_option);
      console.log("Subs ready");
      setIsSelectionDisabled(i == 0);
      //console.log(sub);
    });
  };

  useEffect(() => {
    const getCurrentSub = async () => {
      const sub = await backend.getCurrentSub();
      setCurrentSub(sub);
    };

    const getDashboardList = async () => {
      // console.log(`>>>>>> getDashboardList`);
      const list = await PyBackend.getDashboardList();
      console.log(`>>>>>> getDashboardList: ${list}`);
      setDashboardList(list);
    };

    const getCurrentDashboard = async () => {
      const dashboard = await PyBackend.getCurrentDashboard();
      setCurrentDashboard(dashboard);
    };

    const getConfig = async () => {
      await ApiCallBackend.getConfig().then((res) => {
        console.log(`getConfig: ${JSON.stringify(res.data, null, 2)}`);
        if (res.data.status_code == 200) {
          enabledSkipProxy = res.data.skip_proxy;
          enabledOverrideDNS = res.data.override_dns;
          enhanced_mode = res.data.enhanced_mode;
          allow_remote_access = res.data.allow_remote_access;

          setSkipProxyState(enabledSkipProxy);
          setOverrideDNSState(enabledOverrideDNS);
          setEnhancedMode(enhanced_mode);
          setAllowRemoteAccess(allow_remote_access);
        }
      });
    };

    const loadDate = async () => {
      await getConfig();

      getCurrentSub();
      getDashboardList();
      getCurrentDashboard();
      update_subs();
    };

    loadDate();
  }, []);

  useEffect(() => {
    current_sub = currentSub;
  }, [currentSub]);

  useEffect(() => {
    dashboard_list = dashboardList;
  }, [dashboardList]);

  useEffect(() => {
    current_dashboard = currentDashboard;
  }, [currentDashboard]);

  const enhancedModeOptions = [
    { mode: EnhancedMode.RedirHost, label: "Redir Host" },
    { mode: EnhancedMode.FakeIp, label: "Fake IP" },
  ];

  const enhancedModeNotchLabels: NotchLabel[] = enhancedModeOptions.map(
    (opt, i) => {
      return {
        notchIndex: i,
        label: opt.label,
        value: i,
      };
    }
  );

  const convertEnhancedMode = (value: number) => {
    return enhancedModeOptions[value].mode;
  };

  const convertEnhancedModeValue = (value: EnhancedMode) => {
    return enhancedModeOptions.findIndex((opt) => opt.mode === value);
  };

  return (
    <div>
      <PanelSection title="Service">
        <PanelSectionRow>
          <ToggleField
            label="Enable Clash"
            description={SelectionTips}
            checked={clashState}
            onChange={(value: boolean) => {
              setIsSelectionDisabled(true);
              setSelectionTips("Loading ...");
              backend.resolve(backend.setEnabled(value), (v: boolean) => {
                enabledGlobal = v;
                setIsSelectionDisabled(false);
              });
              //获取 Clash 启动状态
              if (!clashState) {
                let check_running_handle = setInterval(() => {
                  backend.resolve(backend.getRunningStatus(), (v: String) => {
                    // console.log(v);
                    switch (v) {
                      case "Loading":
                        setSelectionTips("Loading ...");
                        break;
                      case "Failed":
                        setSelectionTips(
                          "Failed to start, please check /tmp/tomoon.log"
                        );
                        setClashState(false);
                        break;
                      case "Success":
                        setSelectionTips("Clash is running.");
                        break;
                    }
                    if (v != "Loading") {
                      clearInterval(check_running_handle);
                    }
                  });
                }, 500);
              } else {
                setSelectionTips("Run Clash in background");
              }
              setOptionDropdownDisabled(value);
              setOpenDashboardDisabled(!value);
            }}
            disabled={isSelectionDisabled}
          />
        </PanelSectionRow>
        <PanelSectionRow>
          <DropdownItem
            // disabled={optionDropdownDisabled}
            strDefaultLabel={"Select a Subscription"}
            rgOptions={options}
            selectedOption={currentSub}
            onMenuWillOpen={() => {
              update_subs();
              // setOptions(subs_option);
            }}
            onChange={(x) => {
              const setSub = async () => {
                await backend.setSub(x.data);
                await ApiCallBackend.reloadClashConfig();
              }
              backend.resolve(setSub(), () => {
                setIsSelectionDisabled(false);
              });
            }}
          />
        </PanelSectionRow>
        <PanelSectionRow>
          <ButtonItem
            layout="below"
            onClick={() => {
              Router.CloseSideMenus();
              Router.Navigate("/tomoon-config");
            }}
          >
            Manage Subscriptions
          </ButtonItem>
        </PanelSectionRow>
        <PanelSectionRow>
          <ButtonItem
            layout="below"
            onClick={() => {
              Router.CloseSideMenus();
              let param = "";
              let page = "setup";
              const currentDashboard_name =
                currentDashboard.split("/").pop() || "yacd-meta";
              if (currentDashboard_name) {
                switch (currentDashboard_name) {
                  case "metacubexd":
                    page = "setup";
                    break;
                  default:
                    page = "proxies";
                    break;
                }
                param = `/${currentDashboard_name}/#/${page}?hostname=127.0.0.1&port=9090&secret=`;
              }
              Navigation.NavigateToExternalWeb(
                "http://127.0.0.1:9090/ui" + param
              );
            }}
            disabled={openDashboardDisabled}
          >
            Open Dashboard
          </ButtonItem>
        </PanelSectionRow>
        <PanelSectionRow>
          <DropdownItem
            label={"Select Dashboard"}
            strDefaultLabel={"Select Dashboard"}
            rgOptions={(dashboardList || []).map((path) => {
              return {
                label: path.split("/").pop(),
                data: path,
              };
            })}
            selectedOption={currentDashboard}
            onChange={(val) => {
              console.log(`>>>>>>>>>>>>>>>> selected dashboard: ${val.data}`);
              current_dashboard = val.data;
              PyBackend.setCurrentDashboard(val.data);
            }}
          />
        </PanelSectionRow>
        <PanelSectionRow>
          <ToggleField
            label="Allow Remote Access"
            description="Allow Remote Access to Dashboard"
            checked={allowRemoteAccess}
            onChange={(value: boolean) => {
              ApiCallBackend.allowRemoteAccess(value);
              setAllowRemoteAccess(value);
            }}
          ></ToggleField>
        </PanelSectionRow>
        <PanelSectionRow>
          <ToggleField
            label="Skip Steam Proxy"
            description="Enable for direct Steam downloads"
            checked={skipProxyState}
            onChange={(value: boolean) => {
              ApiCallBackend.skipProxy(value);
              setSkipProxyState(value);
            }}
          ></ToggleField>
        </PanelSectionRow>
        <PanelSectionRow>
          <ToggleField
            label="Override DNS Config"
            description="Force Clash to hijack DNS query"
            checked={overrideDNSState}
            onChange={(value: boolean) => {
              ApiCallBackend.overrideDns(value);
              setOverrideDNSState(value);
            }}
          ></ToggleField>
        </PanelSectionRow>
        {overrideDNSState && (
          <PanelSectionRow>
            <SliderField
              label={"Enhanced Mode"}
              value={convertEnhancedModeValue(enhancedMode)}
              min={0}
              max={enhancedModeNotchLabels.length - 1}
              notchCount={enhancedModeNotchLabels.length}
              notchLabels={enhancedModeNotchLabels}
              notchTicksVisible={true}
              step={1}
              onChange={(value: number) => {
                const _enhancedMode = convertEnhancedMode(value);
                setEnhancedMode(_enhancedMode);
                ApiCallBackend.enhancedMode(_enhancedMode);
              }}
            />
          </PanelSectionRow>
        )}
      </PanelSection>

      <PanelSection title="Tools">
        <PanelSectionRow>
          <ButtonItem
            layout="below"
            onClick={() => {
              backend.resolve(backend.resetNetwork(), () => {
                Router.CloseSideMenus();
                console.log("reset network");
              });
            }}
          >
            Reset Network
          </ButtonItem>
        </PanelSectionRow>
      </PanelSection>
      <VersionComponent />
    </div>
  );
};

const DeckyPluginRouterTest: FC = () => {
  return (
    <SidebarNavigation
      title="To Moon"
      showTitle
      pages={[
        {
          title: "Subscriptions",
          content: <Subscriptions Subscriptions={subs} />,
          route: "/tomoon-config/subscriptions",
        },
        {
          title: "About",
          content: <About />,
          route: "/tomoon-config/about",
        },
        {
          title: "Debug",
          content: <Debug />,
          route: "/tomoon-config/debug",
        },
      ]}
    />
  );
};

export default definePlugin(() => {
  // init USDPL WASM and connection to back-end
  (async function () {
    await backend.initBackend();
    await backend.PyBackend.init();
    usdplReady = true;
    backend.resolve(backend.getEnabled(), (v: boolean) => {
      enabledGlobal = v;
    });
    axios.get("http://127.0.0.1:55556/get_config").then((r) => {
      if (r.data.status_code == 200) {
        enabledSkipProxy = r.data.skip_proxy;
        enabledOverrideDNS = r.data.override_dns;
        enhanced_mode = r.data.enhanced_mode;
        allow_remote_access = r.data.allow_remote_access;
      }
    });
  })();

  routerHook.addRoute("/tomoon-config", DeckyPluginRouterTest);

  return {
    title: <div className={staticClasses.Title}>To Moon</div>,
    content: <Content />,
    icon: <GiEgyptianBird />,
    onDismount() {
      routerHook.removeRoute("/tomoon-config");
    },
  };
});

import {
  ButtonItem,
  definePlugin,
  PanelSection,
  PanelSectionRow,
  Router,
  ServerAPI,
  staticClasses,
  ToggleField,
  SidebarNavigation,
  DropdownOption,
  Navigation,
  DropdownItem,
} from "decky-frontend-lib";
import { VFC, useState } from "react";
import { GiEgyptianBird } from "react-icons/gi";

import {
  Subscriptions,
  About,
  Debug,
  VersionComponent
} from "./pages";

import * as backend from "./backend";
import axios from "axios";

let enabledGlobal = false;
let enabledSkipProxy = false;
let usdplReady = false;
let subs: any[];
let subs_option: any[];


const Content: VFC<{ serverAPI: ServerAPI }> = ({ }) => {

  if (!usdplReady) {
    return (
      <PanelSection>
        Init...
      </PanelSection>
    )
  }
  const [clashState, setClashState] = useState(enabledGlobal);
  backend.resolve(backend.getEnabled(), setClashState);
  axios.get("http://127.0.0.1:55556/get_skip_proxy").then(r => {
    if (r.data.status_code == 200) {
      enabledSkipProxy = r.data.skip_proxy;
    }
  })
  //setInterval(refreshSubOptions, 2000);
  console.log("status :" + clashState);
  let [options, setOptions] = useState<DropdownOption[]>(subs_option);
  const [selectedOption, setSelectedOption] = useState<number | null>(null);
  const [optionDropdownDisabled, setOptionDropdownDisabled] = useState(enabledGlobal);
  const [openDashboardDisabled, setOpenDashboardDisabled] = useState(!enabledGlobal);
  const [isSelectionDisabled, setIsSelectionDisabled] = useState(false);
  const [SelectionTips, setSelectionTips] = useState("Run Clash in background");
  const [skipProxyState, setSkipProxyState] = useState(enabledSkipProxy);

  const update_subs = () => {
    backend.resolve(backend.getSubList(), (v: String) => {
      let x: Array<any> = JSON.parse(v.toString());
      let re = new RegExp("(?<=subs\/).+\.yaml$");
      let i = 0;
      subs = x.map(x => {
        let name = re.exec(x.path);
        return {
          id: i++,
          name: name![0],
          url: x.url
        }
      });
      let items = x.map(x => {
        let name = re.exec(x.path);
        return {
          label: name![0],
          data: x.path
        }
      });
      subs_option = items;
      console.log("Subs ready");
      setIsSelectionDisabled(i == 0);
      //console.log(sub);
    });
  }

  update_subs();

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
                    console.log(v);
                    switch (v) {
                      case "Loading":
                        setSelectionTips("Loading ...");
                        break;
                      case "Failed":
                        setSelectionTips("Failed to start, please check /tmp/tomoon.log");
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
            disabled={optionDropdownDisabled}
            strDefaultLabel="Select a Subscription"
            rgOptions={options}
            selectedOption={selectedOption}
            onMenuWillOpen={() => {
              update_subs();
              setOptions(subs_option);
            }}
            onChange={(x) => {
              backend.resolve(backend.setSub(x.data), () => {
                setIsSelectionDisabled(false);
              });
            }}
          />
        </PanelSectionRow>
        <PanelSectionRow>
          <ButtonItem
            layout="below"
            onClick={() => {
              Router.CloseSideMenus()
              Router.Navigate("/tomoon-config")
            }}
          >
            Manage Subscriptions
          </ButtonItem>
        </PanelSectionRow>
        <PanelSectionRow>
          <ButtonItem
            layout="below"
            onClick={() => {
              Router.CloseSideMenus()
              Navigation.NavigateToExternalWeb("http://127.0.0.1:9090/ui")
              //Router.NavigateToExternalWeb("http://127.0.0.1:9090/ui")
            }}
            disabled={openDashboardDisabled}
          >
            Open Dashboard
          </ButtonItem>
        </PanelSectionRow>
        <PanelSectionRow>
          <ToggleField
            label="Skip Steam Proxy"
            description="Enable for direct Steam downloads"
            checked={skipProxyState}
            onChange={(value: boolean) => {
              axios.post("http://127.0.0.1:55556/skip_proxy", {
                skip_proxy: value
              }, {
                headers: { 'content-type': 'application/x-www-form-urlencoded' },
              });
              setSkipProxyState(value);
            }}
          >
          </ToggleField>
        </PanelSectionRow>
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

const DeckyPluginRouterTest: VFC = () => {
  return (
    <SidebarNavigation
      title="To Moon"
      showTitle
      pages={[
        {
          title: "Subscriptions",
          content: <Subscriptions Subscriptions={subs} />,
          route: "/tomoon-config/subscriptions"
        },
        {
          title: "About",
          content: <About />,
          route: "/tomoon-config/about"
        },
        {
          title: "Debug",
          content: <Debug />,
          route: "/tomoon-config/debug"
        }
      ]}
    />
  );
};

export default definePlugin((serverApi: ServerAPI) => {
  // init USDPL WASM and connection to back-end
  (async function () {
    await backend.initBackend();
    await backend.PyBackend.init(serverApi);
    usdplReady = true;
    backend.resolve(backend.getEnabled(), (v: boolean) => {
      enabledGlobal = v;
    });
    axios.get("http://127.0.0.1:55556/get_skip_proxy").then(r => {
      if (r.data.status_code == 200) {
        enabledSkipProxy = r.data.skip_proxy;
      }
    })
  })();


  serverApi.routerHook.addRoute("/tomoon-config", DeckyPluginRouterTest);

  return {
    title: <div className={staticClasses.Title}>To Moon</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <GiEgyptianBird />,
    onDismount() {
      serverApi.routerHook.removeRoute("/tomoon-config");
    },
  };
});

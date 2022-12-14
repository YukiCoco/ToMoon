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
  Dropdown,
} from "decky-frontend-lib";
import { VFC, useState } from "react";
import { GiEgyptianBird } from "react-icons/gi";

import {
  Subscriptions,
  About,
  Debug
} from "./pages";

import * as backend from "./backend";

let enabledGlobal = false;
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
  //setInterval(refreshSubOptions, 2000);
  console.log("status :" + clashState);
  let [options, setOptions] = useState<DropdownOption[]>(subs_option);
  const [selectedOption, setSelectedOption] = useState<number | null>(null);
  const [optionDropdownDisabled, setOptionDropdownDisabled] = useState(enabledGlobal);
  const [isSelectionDisabled, setIsSelectionDisabled] = useState(false);
  const [SelectionTips, setSelectionTips] = useState("Run Clash in background");

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

  return (
    <PanelSection>
      <PanelSection title="Service">
        <PanelSectionRow>
          <div>
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
                //?????? Clash ????????????
                if (!clashState) {
                  let check_running_handle = setInterval(() => {
                    backend.resolve(backend.getRunningStatus(), (v: String) => {
                      console.log(v);
                      switch (v) {
                        case "Loading":
                          setSelectionTips("Loading ...");
                          break;
                        case "Failed":
                          setSelectionTips("Enable failed, See GitHub for help.");
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
              }}
              disabled={isSelectionDisabled}
            />
          </div>
          <Dropdown
            disabled={optionDropdownDisabled}
            strDefaultLabel="Select a Subscription"
            rgOptions={options}
            selectedOption={selectedOption}
            onMenuWillOpen={() => {
              setOptions(subs_option);
            }}
            onChange={(x) => {
              backend.resolve(backend.setSub(x.data), () => {
                setIsSelectionDisabled(false);
              });
            }}
          />
        </PanelSectionRow>
        {/* <PanelSectionRow>
          
        </PanelSectionRow> */}

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
              Router.NavigateToExternalWeb("http://127.0.0.1:9090/ui")
            }}
          >
            Open Dashboard
          </ButtonItem>
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
    </PanelSection>
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
    usdplReady = true;
    backend.resolve(backend.getEnabled(), (v: boolean) => {
      enabledGlobal = v;
    });
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

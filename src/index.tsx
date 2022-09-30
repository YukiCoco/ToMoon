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
  About
} from "./pages";

import * as backend from "./backend";

let enabledGlobal = false;
let usdplReady = false;
let subs: any[];


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
  let [options, setOptions] = useState<DropdownOption[]>([]);
  const [selectedOption, setSelectedOption] = useState<number | null>(null);
  const [optionDropdownDisabled, setOptionDropdownDisabled] = useState(enabledGlobal);
  const [isSelectionDisabled, setIsSelectionDisabled] = useState(false);

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
              description="Run Clash in background"
              checked={clashState}
              onChange={(value: boolean) => {
                backend.resolve(backend.setEnabled(value), (v: boolean) => {
                  enabledGlobal = v;
                });
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
              //console.log("Getting subs");
              //setOptions(getOptions());
              backend.resolve(backend.getSubList(), (v: String) => {
                let x: Array<any> = JSON.parse(v.toString());
                let re = new RegExp("(?<=subs\/).+\.yaml$");
                let items = x.map(x => {
                  let name = re.exec(x.path);
                  return {
                    label: name![0],
                    data: x.path
                  }
                });
                setOptions(items)
                console.log("refresh subOptions");
                console.log(items);
                //console.log(sub);
              });
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
        <PanelSectionRow>
          {/* {clashState ? "on" : "off"} */}
          {/* <ButtonItem
          layout="below"
          onClick={() => {
            //test
            //var status = false;
            backend.resolve(backend.getEnabled(), (v: boolean) => {
            });
          }}
        >
          TEST BTN
        </ButtonItem> */}
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
    // backend.resolve(backend.getSubList(), (v: String) => {
    //   let x: Array<any> = JSON.parse(v.toString());
    //   let re = new RegExp("(?<=subs\/).+\.yaml$");
    //   let i = 0;
    //   subs = x.map(x => {
    //     let name = re.exec(x.path);
    //     return {
    //       id: i++,
    //       name: name![0],
    //       url: x.url
    //     }
    //   });
    //   console.log("Subs ready");
    //   console.log(subs);
    //   //console.log(sub);
    // });
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

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
import { VFC, useState, useMemo } from "react";
import { FaCat } from "react-icons/fa";

import {
  Subscriptions
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
    console.log(subs);
    //console.log(sub);
  });
  console.log("status :" + clashState);

  const options = useMemo(
    (): DropdownOption[] => [
      {
        data: 1,
        label: "One",
      },
      {
        data: 2,
        label: "Two",
      },
      {
        data: 3,
        label: "Three",
      },
    ],
    []
  );
  const [selectedOption, setSelectedOption] = useState<number | null>(null);

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
              }}
            />
          </div>
          <Dropdown
            strDefaultLabel="Select a Subscription"
            rgOptions={options}
            selectedOption={selectedOption}
            onChange={async (_) => {

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
              Router.Navigate("/clash-config")
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

      <PanelSection title="Debug">
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
      title="Clash Deck"
      showTitle
      pages={[
        {
          title: "Subscriptions",
          content: <Subscriptions Subscriptions={subs} />,
          route: "/clash-config/subscriptions"
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
      console.log(subs);
      //console.log(sub);
    });
  })();


  serverApi.routerHook.addRoute("/clash-config", DeckyPluginRouterTest, {
    exact: true,
  });

  return {
    title: <div className={staticClasses.Title}>Clash Deck</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <FaCat />,
    onDismount() {
      serverApi.routerHook.removeRoute("/clash-config");
    },
  };
});

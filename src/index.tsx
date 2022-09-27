import {
  ButtonItem,
  definePlugin,
  PanelSection,
  PanelSectionRow,
  Router,
  ServerAPI,
  staticClasses,
  ToggleField
} from "decky-frontend-lib";
import { VFC, useState } from "react";
import { FaCat } from "react-icons/fa";

import * as backend from "./backend";

let enabledGlobal = false;
let usdplReady = false;

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
  console.log("status :" + clashState);

  return (
    <PanelSection>
      <PanelSection title="Service">
        <PanelSectionRow>
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
        </PanelSectionRow>
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
    <div style={{ marginTop: "50px", color: "white" }}>
      Hello World!
      {/* <DialogButton onClick={() => Router.NavigateToStore()}>
        Go to Store
      </DialogButton> */}
    </div>
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

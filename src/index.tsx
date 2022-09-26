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

// init USDPL WASM and connection to back-end
(async function () {
  await backend.initBackend();
})();

const Content: VFC<{ serverAPI: ServerAPI }> = ({}) => {

  return (
    <PanelSection>
      <PanelSectionRow>
        <ToggleField
          label="Enable Clash"
          description="Run Clash in background"
          checked={enabledGlobal}
          onChange={(value: boolean) => {
            backend.resolve(backend.setEnabled(value), (v : boolean) => {
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
            Router.NavigateToExternalWeb("http://clash.razord.top")
          }}
        >
          Open Dashboard
        </ButtonItem>
      </PanelSectionRow>
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

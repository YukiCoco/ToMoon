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

const [enabledGlobal, setEnableInternal] = useState<boolean>(false);

let setEnable = (enable: boolean) => {
  setEnableInternal(enable)
}

const Content: VFC<{ serverAPI: ServerAPI }> = ({}) => {

  return (
    <PanelSection title="Panel Section">
      <PanelSectionRow>
        <ToggleField
          label="Enable Clash"
          description="Run Clash in background"
          checked={enabledGlobal}
          onChange={(value: boolean) => {
            backend.resolve(backend.setEnabled(value), setEnable);
          }}
        />
      </PanelSectionRow>
      <PanelSectionRow>
        <ButtonItem
          layout="below"
          onClick={() => {
            //do something here
            Router.NavigateToExternalWeb("http://clash.razord.top")
          }}
        >
          Open Dashboard
        </ButtonItem>
      </PanelSectionRow>
      {/* <PanelSectionRow>
        <ButtonItem
          layout="below"
          onClick={(e) =>
            showContextMenu(
              <Menu label="Menu" cancelText="CAAAANCEL" onCancel={() => {}}>
                <MenuItem onSelected={() => {}}>Item #1</MenuItem>
                <MenuItem onSelected={() => {}}>Item #2</MenuItem>
                <MenuItem onSelected={() => {}}>Item #3</MenuItem>
              </Menu>,
              e.currentTarget ?? window
            )
          }
        >
          Server says yolo
        </ButtonItem>
      </PanelSectionRow>

      <PanelSectionRow>
        <div style={{ display: "flex", justifyContent: "center" }}>
          <img src={logo} />
        </div>
      </PanelSectionRow>

      <PanelSectionRow>
        <ButtonItem
          layout="below"
          onClick={() => {
            Router.CloseSideMenus();
            Router.Navigate("/decky-plugin-test");
          }}
        >
          Router
        </ButtonItem>
      </PanelSectionRow> */}
    </PanelSection>
  );
};

// const DeckyPluginRouterTest: VFC = () => {
//   return (
//     <div style={{ marginTop: "50px", color: "white" }}>
//       Hello World!
//       <DialogButton onClick={() => Router.NavigateToStore()}>
//         Go to Store
//       </DialogButton>
//     </div>
//   );
// };

export default definePlugin((serverApi: ServerAPI) => {
  // serverApi.routerHook.addRoute("/decky-plugin-test", DeckyPluginRouterTest, {
  //   exact: true,
  // });

  return {
    title: <div className={staticClasses.Title}>Clash Deck</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <FaCat />,
    onDismount() {
      //serverApi.routerHook.removeRoute("/decky-plugin-test");
    },
  };
});

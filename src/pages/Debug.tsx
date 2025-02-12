import { FC } from "react";
import { ButtonItem, PanelSectionRow } from "@decky/ui";
import { VscDebug } from "react-icons/vsc";

import * as backend from "../backend/backend";

export const Debug: FC = () => {
  return (
    // The outermost div is to push the content down into the visible area
    <>
      <PanelSectionRow>
        <ButtonItem
          icon={<VscDebug style={{ display: "block" }} />}
          label="Debug"
          onClick={() => {
            backend.resolve(backend.createDebugLog(), () => {});
          }}
          description="Debug Log is located at /tmp/tomoon.debug.log , please send it to the developer."
        >
          Generate Debug Log
        </ButtonItem>
      </PanelSectionRow>
    </>
  );
};

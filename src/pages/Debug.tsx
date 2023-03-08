import { VFC } from "react";
import { ButtonItem, PanelSectionRow } from "decky-frontend-lib";
import { VscDebug } from "react-icons/vsc"

import * as backend from "../backend";

export const Debug: VFC = () => {
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
}
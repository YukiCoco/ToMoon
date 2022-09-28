import { PanelSectionRow, TextField, ButtonItem } from "decky-frontend-lib";
import { useState, VFC } from "react";
import { cleanPadding } from "../style";

export const Subscriptions: VFC = () => {
    const [text, setText] = useState("");
    return (

        <div>
            <style>
                {`
                    #subscription-download-textfiled {
                        padding: 0px !important
                    }
                    #subscription-download-textfiled > div {
                        margin-bottom: 0px !important
                    }
                `}
            </style>
            <PanelSectionRow>
                <div id="subscription-download-textfiled" style={cleanPadding}>
                    <TextField
                        label="Subscription Link"
                        value={text}
                        onChange={(e) => setText(e?.target.value)}
                    />
                </div>
                <ButtonItem layout="below" onClick={() => { }}>
                    Download
                </ButtonItem>
                <ButtonItem layout="below" onClick={() => { }}>
                    Update All
                </ButtonItem>
            </PanelSectionRow>
            <PanelSectionRow>
                <ButtonItem label="红杏出墙" description="http://xxxx.com">Delete</ButtonItem>
                <ButtonItem label="要你命3000" description="http://aaaa.com">Delete</ButtonItem>
            </PanelSectionRow>
        </div >
    );
};

import { PanelSectionRow, TextField, ButtonItem } from "decky-frontend-lib";
import { useReducer, useState, VFC } from "react";
import { cleanPadding } from "../style";
import { SubList } from "./components/SubList";

export const Subscriptions: VFC = () => {
    const [text, setText] = useState("");
    const [subscriptions, updateSubscriptions] = useState([
        {
            id: 0,
            name: "红杏出墙.yaml",
            url: "http://xxx.com"
        },
        {
            id: 1,
            name: "红杏出墙2.yaml",
            url: "http://aaa.com"
        }
    ]);
    const [_, forceUpdate] = useReducer(x => x + 1, 0);
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
                <ButtonItem layout="below" onClick={() => {
                 }}>
                    Update All
                </ButtonItem>
            </PanelSectionRow>
            <PanelSectionRow>
                {/* {
                    subscriptions.map(x => {
                        return (
                            <div>
                                <ButtonItem label={x.name} description={x.url} onClick={
                                    () => {
                                        //删除订阅
                                    }
                                }>Delete</ButtonItem>
                            </div>
                        );
                    })
                } */}
                <SubList Subscriptions={subscriptions} UpdateSub={updateSubscriptions} Refresh={forceUpdate}></SubList>
            </PanelSectionRow>
        </div >
    );
};

import { PanelSectionRow, TextField, ButtonItem } from "decky-frontend-lib";
import { useReducer, useState, VFC } from "react";
import { cleanPadding } from "../style";
import { SubList } from "./components/SubList";

import * as backend from "../backend";

export const Subscriptions: VFC = () => {
    const [text, setText] = useState("");
    const [downloadTips, setDownloadTips] = useState("");
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

    let checkStatusHandler: any;
    const refreshDownloadStatus = () => {
        backend.resolve(backend.getDownloadStatus(), (v: any) => {
            let response = v.toString();
            switch (response) {
                case "Error":
                    setDownloadTips("Download Error");
                    break;
                case "Failed":
                    setDownloadTips("Download Failed");
                    break;
                case "Success":
                    setDownloadTips("Download Succeeded");
                    break;
                default:
                    break;
            }
            if (response != "Downloading") {
                clearInterval(checkStatusHandler);
                console.log("Download successfully");
            }
        });
    }
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
                        description={downloadTips}
                    />
                </div>
                <ButtonItem layout="below" onClick={() => {
                    backend.resolve(backend.downloadSub(text), () => {
                        console.log("download sub: " + text);
                    });
                    checkStatusHandler = setInterval(refreshDownloadStatus, 500);
                }}>
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

import { PanelSectionRow, TextField, ButtonItem } from "decky-frontend-lib";
import { useReducer, useState, VFC } from "react";
import { cleanPadding } from "../style";
import { SubList } from "./components/SubList";

import * as backend from "../backend";

interface SubProp {
    Subscriptions: Array<any>,
}

export const Subscriptions: VFC<SubProp> = ({ Subscriptions }) => {
    const [text, setText] = useState("");
    const [downloadTips, setDownloadTips] = useState("");
    const [subscriptions, updateSubscriptions] = useState(Subscriptions);
    const [downlaodBtnDisable, setDownlaodBtnDisable] = useState(false);
    const [_, forceUpdate] = useReducer(x => x + 1, 0);

    let checkStatusHandler: any;

    const refreshDownloadStatus = () => {
        backend.resolve(backend.getDownloadStatus(), (v: any) => {
            let response = v.toString();
            switch (response) {
                case "Downloading":
                    setDownloadTips("Downloading... Please wait");
                    break;
                case "Error":
                    setDownloadTips("Download Error");
                    break;
                case "Failed":
                    setDownloadTips("Download Failed");
                    break;
                case "Success":
                    setDownloadTips("Download Succeeded");
                    // 刷新 Subs
                    refreshSubs();
                    break;
            }
            if (response != "Downloading") {
                clearInterval(checkStatusHandler);
                setDownlaodBtnDisable(false);
            }
        });
    }

    const refreshSubs = () => {
        backend.resolve(backend.getSubList(), (v: String) => {
            let x: Array<any> = JSON.parse(v.toString());
            let re = new RegExp("(?<=subs\/).+\.yaml$");
            let i = 0;
            let subs = x.map(x => {
                let name = re.exec(x.path);
                return {
                    id: i++,
                    name: name![0],
                    url: x.url
                }
            });
            console.log("Subs refresh");
            updateSubscriptions(subs);
            //console.log(sub);
        });
    }

    console.log("load Subs page");

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
                <ButtonItem layout="below" disabled={downlaodBtnDisable} onClick={() => {
                    setDownlaodBtnDisable(true);
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

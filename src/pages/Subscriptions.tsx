import { PanelSectionRow, TextField, ButtonItem } from "@decky/ui";
import { useReducer, useState, FC } from "react";
import { cleanPadding } from "../style";
import { SubList } from "../components";
import { QRCodeCanvas } from "qrcode.react";

import * as backend from "../backend/backend";
import axios from "axios";

interface SubProp {
  Subscriptions: Array<any>;
}

export const Subscriptions: FC<SubProp> = ({ Subscriptions }) => {
  const [text, setText] = useState("");
  const [downloadTips, setDownloadTips] = useState("");
  const [subscriptions, updateSubscriptions] = useState(Subscriptions);
  const [downlaodBtnDisable, setDownlaodBtnDisable] = useState(false);
  const [updateBtnDisable, setUpdateBtnDisable] = useState(false);
  const [_, forceUpdate] = useReducer((x) => x + 1, 0);
  const [updateTips, setUpdateTips] = useState("");
  const [QRPageUrl, setQRPageUrl] = useState("");

  let checkStatusHandler: any;
  let checkUpdateStatusHandler: any;

  const refreshDownloadStatus = () => {
    backend.resolve(backend.getDownloadStatus(), (v: any) => {
      let response = v.toString();
      switch (response) {
        case "Downloading":
          setDownloadTips("Downloading...");
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
  };

  const refreshUpdateStatus = () => {
    backend.resolve(backend.getUpdateStatus(), (v: any) => {
      let response = v.toString();
      switch (response) {
        case "Downloading":
          setDownloadTips("Downloading... Please wait");
          break;
        case "Error":
          setDownloadTips("Update Error");
          break;
        case "Failed":
          setDownloadTips("Update Failed");
          break;
        case "Success":
          setDownloadTips("Update Succeeded");
          // 刷新 Subs
          refreshSubs();
          break;
      }
      if (response != "Downloading") {
        clearInterval(checkUpdateStatusHandler);
        setUpdateBtnDisable(false);
      }
    });
  };

  const refreshSubs = () => {
    backend.resolve(backend.getSubList(), (v: String) => {
      let x: Array<any> = JSON.parse(v.toString());
      let re = new RegExp("(?<=subs/).+.yaml$");
      let i = 0;
      let subs = x.map((x) => {
        let name = re.exec(x.path);
        return {
          id: i++,
          name: name![0],
          url: x.url,
        };
      });
      console.log("Subs refresh");
      updateSubscriptions(subs);
      //console.log(sub);
    });
  };

  //获取 QR Page
  axios.get("http://127.0.0.1:55556/get_ip_address").then((r) => {
    if (r.data.status_code == 200) {
      setQRPageUrl(`http://${r.data.ip}:55556`);
    } else {
      setQRPageUrl("");
    }
  });

  console.log("load Subs page");

  return (
    <>
      <style>
        {`
                    #subscription-download-textfiled {
                        padding: 0px !important
                    }
                    #subscription-download-textfiled > div {
                        margin-bottom: 0px !important
                    }
                    #subscription-qrcode {
                        display: flex;
                        justify-content: center;
                    }
                `}
      </style>
      <PanelSectionRow>
        <div id="subscription-qrcode">
          <QRCodeCanvas value={QRPageUrl} size={128} />
        </div>
        <div id="subscription-download-textfiled" style={cleanPadding}>
          <TextField
            label="Subscription Link"
            value={text}
            onChange={(e) => setText(e?.target.value)}
            description={downloadTips}
          />
        </div>
        <ButtonItem
          layout="below"
          disabled={downlaodBtnDisable}
          onClick={() => {
            setDownlaodBtnDisable(true);
            backend.resolve(backend.downloadSub(text), () => {
              console.log("download sub: " + text);
            });
            checkStatusHandler = setInterval(refreshDownloadStatus, 500);
          }}
        >
          Download
        </ButtonItem>
        <ButtonItem
          layout="below"
          description={updateTips}
          onClick={() => {
            setUpdateBtnDisable(true);
            backend.resolve(backend.updateSubs(), () => {
              console.log("update subs.");
            });
            checkUpdateStatusHandler = setInterval(refreshUpdateStatus, 500);
          }}
          disabled={updateBtnDisable}
        >
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
        <SubList
          Subscriptions={subscriptions}
          UpdateSub={updateSubscriptions}
          Refresh={forceUpdate}
        ></SubList>
      </PanelSectionRow>
    </>
  );
};

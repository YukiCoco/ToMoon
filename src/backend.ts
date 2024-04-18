import { ServerAPI } from "decky-frontend-lib";
import { init_usdpl, init_embedded, call_backend } from "usdpl-front";

const USDPL_PORT: number = 55555;

// Utility

export function resolve(promise: Promise<any>, setter: any) {
  (async function () {
    let data = await promise;
    if (data != null) {
      console.debug("Got resolved", data);
      setter(data);
    } else {
      console.warn("Resolve failed:", data);
    }
  })();
}

export function execute(promise: Promise<any[]>) {
  (async function () {
    let data = await promise;
    console.debug("Got executed", data);
  })();
}

export async function initBackend() {
  // init usdpl
  await init_embedded();
  init_usdpl(USDPL_PORT);
  //setReady(true);
}

// Back-end functions

export async function setEnabled(value: boolean): Promise<boolean> {
  return (await call_backend("set_clash_status", [value]))[0];
}

export async function getEnabled(): Promise<boolean> {
  return (await call_backend("get_clash_status", []))[0];
}

export async function resetNetwork(): Promise<any[]> {
  return await call_backend("reset_network", []);
}

export async function downloadSub(value: String): Promise<any[]> {
  return (await call_backend("download_sub", [value]))[0];
}

export async function getDownloadStatus(): Promise<String> {
  return (await call_backend("get_download_status", []))[0];
}

export async function getSubList(): Promise<String> {
  return (await call_backend("get_sub_list", []))[0];
}

export async function deleteSub(value: Number): Promise<any> {
  return (await call_backend("delete_sub", [value]))[0];
}

export async function setSub(value: String): Promise<any> {
  return (await call_backend("set_sub", [value]))[0];
}

export async function updateSubs(): Promise<any> {
  return (await call_backend("update_subs", []))[0];
}

export async function getUpdateStatus(): Promise<String> {
  return (await call_backend("get_update_status", []))[0];
}

export async function createDebugLog(): Promise<boolean> {
  return (await call_backend("create_debug_log", []))[0];
}

export async function getRunningStatus(): Promise<String> {
  return (await call_backend("get_running_status", []))[0];
}

export class PyBackendData {
  private serverAPI: ServerAPI | undefined;
  private current_version = "";
  private latest_version = "";

  public async init(serverAPI: ServerAPI) {
    this.serverAPI = serverAPI;

    await this.serverAPI!.callPluginMethod<{}, string>("get_version", {}).then(
      (res) => {
        if (res.success) {
          console.info("current_version = " + res.result);
          this.current_version = res.result;
        }
      }
    );

    await this.serverAPI!.callPluginMethod<{}, string>(
      "get_latest_version",
      {}
    ).then((res) => {
      if (res.success) {
        console.info("latest_version = " + res.result);
        this.latest_version = res.result;
      }
    });
  }

  public getCurrentVersion() {
    return this.current_version;
  }

  public setCurrentVersion(version: string) {
    this.current_version = version;
  }

  public getLatestVersion() {
    return this.latest_version;
  }

  public setLatestVersion(version: string) {
    this.latest_version = version;
  }
}

export class PyBackend {
  private static serverAPI: ServerAPI;
  public static data: PyBackendData;

  public static async init(serverAPI: ServerAPI) {
    this.serverAPI = serverAPI;
    this.data = new PyBackendData();
    this.data.init(serverAPI);
  }

  public static async getLatestVersion(): Promise<string> {
    const version = (
      await this.serverAPI!.callPluginMethod("get_latest_version", {})
    ).result as string;
    
    const versionReg = /^\d+\.\d+\.\d+$/;
    if (!versionReg.test(version)) {
      return "";
    }
    return version;
  }

  // updateLatest
  public static async updateLatest() {
    await this.serverAPI!.callPluginMethod("update_latest", {});
  }

  // get_version
  public static async getVersion() {
    return (await this.serverAPI!.callPluginMethod("get_version", {}))
      .result as string;
  }
}

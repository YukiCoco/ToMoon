use std::{fs, string, thread};

use crate::{control::DownloadStatus, helper, settings::Subscription};

use super::control::ControlRuntime;

use rand::{distributions::Alphanumeric, Rng};
use usdpl_back::core::serdes::Primitive;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn get_clash_status(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_settings = runtime.settings_clone();
    move |_| {
        let lock = match runtime_settings.read() {
            Ok(x) => x,
            Err(e) => {
                log::error!("get_enable failed to acquire settings read lock: {}", e);
                return vec![];
            }
        };
        log::debug!("get_enable() success");
        log::info!("get clash status with {}", lock.enable);
        vec![lock.enable.into()]
    }
}

pub fn set_clash_status(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_settings = runtime.settings_clone();
    let runtime_state = runtime.state_clone();
    let clash = runtime.clash_state_clone();
    move |params| {
        if let Some(Primitive::Bool(enabled)) = params.get(0) {
            let mut settings = match runtime_settings.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("set_enable failed to acquire settings write lock: {}", e);
                    return vec![];
                }
            };
            log::info!("set clash status to {}", enabled);
            if settings.enable != *enabled {
                let mut clash = match clash.write() {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("set_enable failed to acquire state write lock: {}", e);
                        return vec![];
                    }
                };
                // Enable Clash
                if *enabled {
                    match clash.run() {
                        Ok(_) => (),
                        Err(e) => {
                            log::error!("Run clash error: {}", e);
                        }
                    }
                } else {
                    // Disable Clash
                    // TODO: 关闭错误处理
                    clash.stop();
                }
                settings.enable = *enabled;
                let mut state = match runtime_state.write() {
                    Ok(x) => x,
                    Err(e) => {
                        log::error!("set_enable failed to acquire state write lock: {}", e);
                        return vec![];
                    }
                };
                state.dirty = true;
                log::debug!("set_enable({}) success", enabled);
            }
            vec![(*enabled).into()]
        } else {
            Vec::new()
        }
    }
}

pub fn reset_network() -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    |_| {
        match helper::reset_system_network() {
            Ok(_) => (),
            Err(e) => {
                log::error!("Error occured while reset_network() : {}", e);
                return vec![];
            }
        }
        log::info!("Successfully reset network");
        return vec![];
    }
}

pub fn download_sub(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let download_status = runtime.downlaod_status_clone();
    let runtime_state = runtime.state_clone();
    let runtime_setting = runtime.settings_clone();
    move |params| {
        if let Some(Primitive::String(url)) = params.get(0) {
            match download_status.write() {
                Ok(mut x) => {
                    let path = match runtime_state.read() {
                        Ok(x) => x.home.as_path().join(".config/clashdeck/subs/"),
                        Err(e) => {
                            log::error!("download_sub() faild to acquire state read {}", e);
                            return vec![];
                        }
                    };
                    *x = DownloadStatus::Downloading;
                    //新线程复制准备
                    let url = url.clone();
                    let download_status = download_status.clone();
                    let runtime_setting = runtime_setting.clone();
                    let runtime_state = runtime_state.clone();
                    //开始下载
                    thread::spawn(move || {
                        let update_status = |status: DownloadStatus| {
                            //修改下载状态
                            match download_status.write() {
                                Ok(mut x) => {
                                    *x = status;
                                }
                                Err(e) => {
                                    log::error!(
                                        "download_sub() faild to acquire download_status write {}",
                                        e
                                    );
                                }
                            }
                        };
                        match minreq::get(url.clone()).with_timeout(10).send() {
                            Ok(x) => {
                                let response = x.as_str().unwrap();
                                let yaml = match serde_yaml::from_str::<serde_yaml::Value>(response)
                                {
                                    Ok(x) => x,
                                    Err(e) => {
                                        log::error!("The downlaoded sub is not a legal profile.");
                                        log::error!("Error Message:{}", e);
                                        update_status(DownloadStatus::Error);
                                        return;
                                    }
                                };

                                if let Some(x) = yaml.as_mapping() {
                                    if !x.contains_key("rules") {
                                        log::error!("Cannt found rules, the downlaoded sub is not a legal profile.");
                                        update_status(DownloadStatus::Error);
                                        return;
                                    }
                                } else {
                                    log::error!("Cannt mapping yaml, The downlaoded sub is not a legal profile.");
                                    update_status(DownloadStatus::Error);
                                    return;
                                }

                                let s: String = rand::thread_rng()
                                    .sample_iter(&Alphanumeric)
                                    .take(5)
                                    .map(char::from)
                                    .collect();
                                let path = path.join(s + ".yaml");
                                //保存订阅
                                if let Some(parent) = path.parent() {
                                    if let Err(e) = std::fs::create_dir_all(parent) {
                                        log::error!("Failed while creating sub dir.");
                                        log::error!("Error Message:{}", e);
                                        update_status(DownloadStatus::Error);
                                        return;
                                    }
                                }
                                let path = path.to_str().unwrap();
                                if let Err(e) = fs::write(path, response) {
                                    log::error!("Failed while saving sub.");
                                    log::error!("Error Message:{}", e);
                                }
                                //下载成功
                                //修改下载状态
                                log::info!("Download profile successfully.");
                                update_status(DownloadStatus::Success);
                                //存入设置
                                match runtime_setting.write() {
                                    Ok(mut x) => {
                                        x.subscriptions
                                            .push(Subscription::new(path.to_string(), url));
                                        let mut state = match runtime_state.write() {
                                            Ok(x) => x,
                                            Err(e) => {
                                                log::error!("set_enable failed to acquire state write lock: {}", e);
                                                return;
                                            }
                                        };
                                        state.dirty = true;
                                    }
                                    Err(e) => {
                                        log::error!(
                                        "download_sub() faild to acquire runtime_setting write {}",
                                        e
                                    );
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed while downloading sub.");
                                log::error!("Error Message:{}", e);
                                update_status(DownloadStatus::Failed);
                            }
                        };
                    });
                }
                Err(_) => {
                    log::error!("download_sub() faild to acquire state write");
                    return vec![];
                }
            }
        } else {
        }
        return vec![];
    }
}

pub fn get_download_status(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let downlaod_status = runtime.downlaod_status_clone();
    move |_| {
        match downlaod_status.read() {
            Ok(x) => {
                let status = x.to_string();
                return vec![status.into()];
            }
            Err(e) => {
                log::error!("Error occured while get_download_status()");
            }
        }
        return vec![];
    }
}

pub fn get_sub_list(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_setting = runtime.settings_clone();
    move |_| {
        match runtime_setting.read() {
            Ok(x) => {
                match serde_json::to_string(&x.subscriptions) {
                    Ok(x) => {
                        //返回 json 编码的订阅
                        return vec![x.into()];
                    }
                    Err(e) => {
                        log::error!("Error while serializing data structures");
                        log::error!("Error message: {}", e);
                        return vec![];
                    }
                };
            }
            Err(e) => {
                log::error!(
                    "download_sub() faild to acquire runtime_setting write {}",
                    e
                );
            }
        }
        return vec![];
    }
}

pub fn delete_sub(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_setting = runtime.settings_clone();
    let runtime_state = runtime.state_clone();
    move |params| {
        if let Some(Primitive::F64(id)) = params.get(0) {
            match runtime_setting.write() {
                Ok(mut x) => {
                    if let Some(item) = x.subscriptions.get(*id as usize) {
                        match fs::remove_file(item.path.as_str()) {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("delete file error: {}", e);
                            }
                        }
                    }
                    log::info!("delete {:?}", x.subscriptions.get(*id as usize).unwrap());
                    x.subscriptions.remove(*id as usize);
                    drop(x);
                    let mut state = match runtime_state.write() {
                        Ok(x) => x,
                        Err(e) => {
                            log::error!("set_enable failed to acquire state write lock: {}", e);
                            return vec![];
                        }
                    };
                    state.dirty = true;
                }
                Err(e) => {
                    log::error!("delete_sub() faild to acquire runtime_setting write {}", e);
                }
            }
        }
        return vec![];
    }
}

pub fn set_sub(runtime: &ControlRuntime) -> impl Fn(Vec<Primitive>) -> Vec<Primitive> {
    let runtime_clash = runtime.clash_state_clone();
    let runtime_state = runtime.state_clone();
    let runtime_setting = runtime.settings_clone();
    move |params: Vec<Primitive>| {
        if let Some(Primitive::String(path)) = params.get(0) {
            //更新到配置文件中
            match runtime_setting.write() {
                Ok(mut x) => {
                    x.current_sub = (*path).clone();
                    let mut state = match runtime_state.write() {
                        Ok(x) => x,
                        Err(e) => {
                            log::error!("set_sub failed to acquire state write lock: {}", e);
                            return vec![];
                        }
                    };
                    state.dirty = true;
                    drop(x);
                    drop(state);
                }
                Err(e) => {
                    log::error!("get_enable failed to acquire settings read lock: {}", e);
                    return vec![];
                }
            };
            //更新到当前内存中
            match runtime_clash.write() {
                Ok(mut x) => {
                    x.update_config_path(path);
                    log::info!("set profile path to {}", path);
                }
                Err(e) => {
                    log::error!("set_sub() failed to acquire clash write lock: {}", e);
                }
            }
        }
        return vec![];
    }
}

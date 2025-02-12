use actix_web::{body::BoxBody, web, HttpResponse, Result};
use local_ip_address::local_ip;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{collections::HashMap, fs, path::PathBuf, sync::Mutex};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::time::sleep;

use crate::{
    control::{ClashError, ClashErrorKind, EnhancedMode},
    helper,
    settings::State,
};

pub struct Runtime(pub *const crate::control::ControlRuntime);
unsafe impl Send for Runtime {}

pub struct AppState {
    pub link_table: Mutex<HashMap<u16, String>>,
    pub runtime: Mutex<Runtime>,
}

#[derive(Deserialize)]
pub struct GenLinkParams {
    link: String,
    subconv: bool,
}

#[derive(Deserialize)]
pub struct SkipProxyParams {
    skip_proxy: bool,
}

#[derive(Deserialize)]
pub struct AllowRemoteAccessParams {
    allow_remote_access: bool,
}

#[derive(Deserialize)]
pub struct OverrideDNSParams {
    override_dns: bool,
}

#[derive(Deserialize)]
pub struct EnhancedModeParams {
    enhanced_mode: EnhancedMode,
}

#[derive(Serialize, Deserialize)]
pub struct GenLinkResponse {
    status_code: u16,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SkipProxyResponse {
    status_code: u16,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct OverrideDNSResponse {
    status_code: u16,
    message: String,
}

#[derive(Deserialize)]
pub struct GetLinkParams {
    code: u16,
}
#[derive(Serialize, Deserialize)]
pub struct GetLinkResponse {
    status_code: u16,
    link: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetConfigResponse {
    status_code: u16,
    skip_proxy: bool,
    override_dns: bool,
    enhanced_mode: EnhancedMode,
    allow_remote_access: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetLocalIpAddressResponse {
    status_code: u16,
    ip: Option<String>,
}

impl actix_web::ResponseError for ClashError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        if self.ErrorKind == ClashErrorKind::ConfigNotFound {
            actix_web::http::StatusCode::NOT_FOUND
        } else {
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let mut res = HttpResponse::new(self.status_code());
        let mime = "text/plain; charset=utf-8";
        res.headers_mut().insert(
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::HeaderValue::from_str(mime).unwrap(),
        );
        res.set_body(BoxBody::new(self.Message.clone()))
    }
}

pub async fn skip_proxy(
    state: web::Data<AppState>,
    params: web::Form<SkipProxyParams>,
) -> Result<HttpResponse> {
    let skip_proxy = params.skip_proxy.clone();
    let runtime = state.runtime.lock().unwrap();
    let runtime_settings;
    let runtime_state;
    unsafe {
        let runtime = runtime.0.as_ref().unwrap();
        runtime_settings = runtime.settings_clone();
        runtime_state = runtime.state_clone();
    }
    match runtime_settings.write() {
        Ok(mut x) => {
            x.skip_proxy = skip_proxy;
            let mut state = match runtime_state.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("set_enable failed to acquire state write lock: {}", e);
                    return Err(actix_web::Error::from(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::InnerError,
                    }));
                }
            };
            state.dirty = true;
        }
        Err(e) => {
            log::error!("Failed while toggle skip Steam proxy.");
            log::error!("Error Message:{}", e);
            return Err(actix_web::Error::from(ClashError {
                Message: e.to_string(),
                ErrorKind: ClashErrorKind::ConfigNotFound,
            }));
        }
    }
    let r = SkipProxyResponse {
        message: "修改成功".to_string(),
        status_code: 200,
    };
    Ok(HttpResponse::Ok().json(r))
}

pub async fn override_dns(
    state: web::Data<AppState>,
    params: web::Form<OverrideDNSParams>,
) -> Result<HttpResponse> {
    let override_dns = params.override_dns.clone();
    let runtime = state.runtime.lock().unwrap();
    let runtime_settings;
    let runtime_state;
    unsafe {
        let runtime = runtime.0.as_ref().unwrap();
        runtime_settings = runtime.settings_clone();
        runtime_state = runtime.state_clone();
    }
    match runtime_settings.write() {
        Ok(mut x) => {
            x.override_dns = override_dns;
            let mut state = match runtime_state.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("override_dns failed to acquire state write lock: {}", e);
                    return Err(actix_web::Error::from(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::InnerError,
                    }));
                }
            };
            state.dirty = true;
        }
        Err(e) => {
            log::error!("Failed while toggle override dns.");
            log::error!("Error Message:{}", e);
            return Err(actix_web::Error::from(ClashError {
                Message: e.to_string(),
                ErrorKind: ClashErrorKind::ConfigNotFound,
            }));
        }
    }
    let r = OverrideDNSResponse {
        message: "修改成功".to_string(),
        status_code: 200,
    };
    Ok(HttpResponse::Ok().json(r))
}

// allow_remote_access
pub async fn allow_remote_access(
    state: web::Data<AppState>,
    params: web::Form<AllowRemoteAccessParams>,
) -> Result<HttpResponse> {
    let allow_remote_access = params.allow_remote_access.clone();
    let runtime = state.runtime.lock().unwrap();
    let runtime_settings;
    let runtime_state;
    unsafe {
        let runtime = runtime.0.as_ref().unwrap();
        runtime_settings = runtime.settings_clone();
        runtime_state = runtime.state_clone();
    }
    match runtime_settings.write() {
        Ok(mut x) => {
            x.allow_remote_access = allow_remote_access;
            let mut state = match runtime_state.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!(
                        "allow_remote_access failed to acquire state write lock: {}",
                        e
                    );
                    return Err(actix_web::Error::from(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::InnerError,
                    }));
                }
            };
            state.dirty = true;
        }
        Err(e) => {
            log::error!("Failed while toggle allow_remote_access.");
            log::error!("Error Message:{}", e);
            return Err(actix_web::Error::from(ClashError {
                Message: e.to_string(),
                ErrorKind: ClashErrorKind::ConfigNotFound,
            }));
        }
    }
    let r = OverrideDNSResponse {
        message: "修改成功".to_string(),
        status_code: 200,
    };
    Ok(HttpResponse::Ok().json(r))
}

pub async fn enhanced_mode(
    state: web::Data<AppState>,
    params: web::Form<EnhancedModeParams>,
) -> Result<HttpResponse> {
    let enhanced_mode = params.enhanced_mode.clone();
    let runtime = state.runtime.lock().unwrap();
    let runtime_settings;
    let runtime_state;
    unsafe {
        let runtime = runtime.0.as_ref().unwrap();
        runtime_settings = runtime.settings_clone();
        runtime_state = runtime.state_clone();
    }
    match runtime_settings.write() {
        Ok(mut x) => {
            x.enhanced_mode = enhanced_mode;
            let mut state = match runtime_state.write() {
                Ok(x) => x,
                Err(e) => {
                    log::error!("enhanced_mode failed to acquire state write lock: {}", e);
                    return Err(actix_web::Error::from(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::InnerError,
                    }));
                }
            };
            state.dirty = true;
        }
        Err(e) => {
            log::error!("Failed while toggle enhanced mode.");
            log::error!("Error Message:{}", e);
            return Err(actix_web::Error::from(ClashError {
                Message: e.to_string(),
                ErrorKind: ClashErrorKind::ConfigNotFound,
            }));
        }
    }
    let r = OverrideDNSResponse {
        message: "修改成功".to_string(),
        status_code: 200,
    };
    Ok(HttpResponse::Ok().json(r))
}

pub async fn get_config(state: web::Data<AppState>) -> Result<HttpResponse> {
    let runtime = state.runtime.lock().unwrap();
    let runtime_settings;
    unsafe {
        let runtime = runtime.0.as_ref().unwrap();
        runtime_settings = runtime.settings_clone();
    }
    match runtime_settings.read() {
        Ok(x) => {
            let r = GetConfigResponse {
                skip_proxy: x.skip_proxy,
                override_dns: x.override_dns,
                enhanced_mode: x.enhanced_mode,
                allow_remote_access: x.allow_remote_access,
                status_code: 200,
            };
            return Ok(HttpResponse::Ok().json(r));
        }
        Err(e) => {
            log::error!("Failed while geting skip Steam proxy.");
            log::error!("Error Message:{}", e);
            return Err(actix_web::Error::from(ClashError {
                Message: e.to_string(),
                ErrorKind: ClashErrorKind::ConfigNotFound,
            }));
        }
    };
}

pub async fn reload_clash_config(state: web::Data<AppState>) -> Result<HttpResponse> {
    let runtime = state.runtime.lock().unwrap();
    let runtime_settings;
    let clash_state;
    unsafe {
        let runtime = runtime.0.as_ref().unwrap();
        runtime_settings = runtime.settings_clone();
        clash_state = runtime.clash_state_clone();
    }

    let clash = match clash_state.write() {
        Ok(x) => x,
        Err(e) => {
            log::error!("read clash_state failed: {}", e);
            return Err(actix_web::Error::from(ClashError {
                Message: e.to_string(),
                ErrorKind: ClashErrorKind::InnerError,
            }));
        }
    };

    let settings = match runtime_settings.write() {
        Ok(x) => x,
        Err(e) => {
            log::error!("read runtime_settings failed: {}", e);
            return Err(actix_web::Error::from(ClashError {
                Message: e.to_string(),
                ErrorKind: ClashErrorKind::InnerError,
            }));
        }
    };

    match clash.change_config(
                        settings.skip_proxy, 
                        settings.override_dns,
                        settings.allow_remote_access,
                        settings.enhanced_mode
    ) {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed while change clash config.");
            log::error!("Error Message:{}", e);
            return Err(actix_web::Error::from(ClashError {
                Message: e.to_string(),
                ErrorKind: ClashErrorKind::InnerError,
            }));
        }
    }

    match clash.reload_config().await {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed while reload clash config.");
            log::error!("Error Message:{}", e);
            return Err(actix_web::Error::from(ClashError {
                Message: e.to_string(),
                ErrorKind: ClashErrorKind::InnerError,
            }));
        }
    }
    
    let r = GenLinkResponse {
        message: "重载成功".to_string(),
        status_code: 200,
    };
    Ok(HttpResponse::Ok().json(r))
}

pub async fn download_sub(
    state: web::Data<AppState>,
    params: web::Form<GenLinkParams>,
) -> Result<HttpResponse> {
    let mut url = params.link.clone();
    let subconv = params.subconv.clone();
    let runtime = state.runtime.lock().unwrap();

    let runtime_settings;
    let runtime_state;
    unsafe {
        let runtime = runtime.0.as_ref().unwrap();
        runtime_settings = runtime.settings_clone();
        runtime_state = runtime.state_clone();
    }

    let home = match runtime_state.read() {
        Ok(state) => state.home.clone(),
        Err(_) => State::default().home,
    };
    let path: PathBuf = home.join(".config/tomoon/subs/");

    //是一个本地文件
    if let Some(local_file) = helper::get_file_path(url.clone()) {
        let local_file = PathBuf::from(local_file);
        let filename = (|| -> Result<String, ()> {
            // 如果文件名可被读取则采用
            let mut filename = String::from(local_file.file_name().ok_or(())?.to_str().ok_or(())?);
            if !filename.to_lowercase().ends_with(".yaml")
                && !filename.to_lowercase().ends_with(".yml")
            {
                filename += ".yaml";
            }
            Ok(filename)
        })()
        .unwrap_or({
            log::warn!("The subscription does not have a proper file name.");
            // 否则采用随机名字
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect::<String>()
                + ".yaml"
        });
        if local_file.exists() {
            let file_content = match fs::read_to_string(local_file) {
                Ok(x) => x,
                Err(e) => {
                    log::error!("Failed while creating sub dir.");
                    log::error!("Error Message:{}", e);
                    return Err(actix_web::Error::from(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::ConfigNotFound,
                    }));
                }
            };
            if !helper::check_yaml(&file_content) {
                log::error!("The downloaded subscription is not a legal profile.");
                return Err(actix_web::Error::from(ClashError {
                    Message: "The downloaded subscription is not a legal profile.".to_string(),
                    ErrorKind: ClashErrorKind::ConfigFormatError,
                }));
            }
            //保存订阅
            let path = path.join(filename);
            if let Some(parent) = path.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    log::error!("Failed while creating sub dir.");
                    log::error!("Error Message:{}", e);
                    return Err(actix_web::Error::from(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::InnerError,
                    }));
                }
            }
            let path = path.to_str().unwrap();
            if let Err(e) = fs::write(path, file_content) {
                log::error!("Failed while saving sub, path: {}", path);
                log::error!("Error Message:{}", e);
                return Err(actix_web::Error::from(ClashError {
                    Message: e.to_string(),
                    ErrorKind: ClashErrorKind::InnerError,
                }));
            }
            //修改下载状态
            log::info!("Download profile successfully.");
            //存入设置
            match runtime_settings.write() {
                Ok(mut x) => {
                    x.subscriptions.push(crate::settings::Subscription::new(
                        path.to_string(),
                        url.clone(),
                    ));
                    let mut state = match runtime_state.write() {
                        Ok(x) => x,
                        Err(e) => {
                            log::error!("set_enable failed to acquire state write lock: {}", e);
                            return Err(actix_web::Error::from(ClashError {
                                Message: e.to_string(),
                                ErrorKind: ClashErrorKind::InnerError,
                            }));
                        }
                    };
                    state.dirty = true;
                }
                Err(e) => {
                    log::error!(
                        "download_sub() faild to acquire runtime_setting write {}",
                        e
                    );
                    return Err(actix_web::Error::from(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::InnerError,
                    }));
                }
            };
        } else {
            log::error!("Cannt found file {}", local_file.to_str().unwrap());
            return Err(actix_web::Error::from(ClashError {
                Message: format!("Cannt found file {}", local_file.to_str().unwrap()),
                ErrorKind: ClashErrorKind::InnerError,
            }));
        }
        // 是一个链接
    } else {
        if subconv {
            let base_url = "http://127.0.0.1:25500/sub";
            let target = "clash";
            let config = "http://127.0.0.1:55556/ACL4SSR_Online.ini";

            // 对参数进行 URL 编码
            let encoded_url = urlencoding::encode(url.as_str());
            let encoded_config = urlencoding::encode(config);

            // 构建请求 URL
            url = format!(
                "{}?target={}&url={}&insert=false&config={}&emoji=true&list=false&tfo=false&scv=true&fdn=false&expand=true&sort=false&new_name=true",
                base_url, target, encoded_url, encoded_config
            );
        }
        match minreq::get(url.clone())
            .with_header(
                "User-Agent",
                format!(
                    "ToMoon/{} mihomo/1.18.3 Clash/v1.18.0",
                    env!("CARGO_PKG_VERSION")
                ),
            )
            .with_timeout(120)
            .send()
        {
            Ok(x) => {
                let response = x.as_str().unwrap();

                if !helper::check_yaml(&String::from(response)) {
                    log::error!("The downloaded subscription is not a legal profile.");
                    return Err(actix_web::Error::from(ClashError {
                        Message: "The downloaded subscription is not a legal profile.".to_string(),
                        ErrorKind: ClashErrorKind::ConfigFormatError,
                    }));
                }
                let filename = x.headers.get("content-disposition");
                let filename = match filename {
                    Some(x) => {
                        let filename = x.split("filename=").collect::<Vec<&str>>()[1]
                            .split(";")
                            .collect::<Vec<&str>>()[0]
                            .replace("\"", "");
                        filename.to_string()
                    }
                    None => {
                        let slash_split = *url.split("/").collect::<Vec<&str>>().last().unwrap();
                        slash_split
                            .split("?")
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap()
                            .to_string()
                    }
                };
                let filename = if filename.is_empty() {
                    log::warn!("The downloaded subscription does not have a file name.");
                    gen_random_name()
                } else {
                    filename
                };
                let filename = if filename.to_lowercase().ends_with(".yaml")
                    || filename.to_lowercase().ends_with(".yml")
                {
                    filename
                } else {
                    filename + ".yaml"
                };
                let mut path = path.join(filename);
                if fs::metadata(&path).is_ok() {
                    path = path.parent().unwrap().join(gen_random_name() + ".yaml");
                }
                //保存订阅
                if let Some(parent) = path.parent() {
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        log::error!("Failed while creating sub dir.");
                        log::error!("Error Message:{}", e);
                        return Err(actix_web::Error::from(ClashError {
                            Message: e.to_string(),
                            ErrorKind: ClashErrorKind::InnerError,
                        }));
                    }
                }
                let path = path.to_str().unwrap();
                log::info!("Writing to path: {}", path);
                if let Err(e) = fs::write(path, response) {
                    log::error!("Failed while saving sub.");
                    log::error!("Error Message:{}", e);
                    return Err(actix_web::Error::from(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::InnerError,
                    }));
                }
                //下载成功
                //修改下载状态
                log::info!("Download profile successfully.");
                //存入设置
                match runtime_settings.write() {
                    Ok(mut x) => {
                        x.subscriptions
                            .push(crate::settings::Subscription::new(path.to_string(), url));
                        let mut state = match runtime_state.write() {
                            Ok(x) => x,
                            Err(e) => {
                                log::error!("set_enable failed to acquire state write lock: {}", e);
                                return Err(actix_web::Error::from(ClashError {
                                    Message: e.to_string(),
                                    ErrorKind: ClashErrorKind::InnerError,
                                }));
                            }
                        };
                        state.dirty = true;
                    }
                    Err(e) => {
                        log::error!(
                            "download_sub() faild to acquire runtime_setting write {}",
                            e
                        );
                        return Err(actix_web::Error::from(ClashError {
                            Message: e.to_string(),
                            ErrorKind: ClashErrorKind::InnerError,
                        }));
                    }
                }
            }
            Err(e) => {
                log::error!("Failed while downloading sub.");
                log::error!("Error Message:{}", e);
                return Err(actix_web::Error::from(ClashError {
                    Message: e.to_string(),
                    ErrorKind: ClashErrorKind::NetworkError,
                }));
            }
        };
    }
    let r = GenLinkResponse {
        message: "下载成功".to_string(),
        status_code: 200,
    };
    Ok(HttpResponse::Ok().json(r))
}

fn gen_random_name() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect()
}

pub async fn get_link(
    state: web::Data<AppState>,
    info: web::Query<GetLinkParams>,
) -> Result<web::Json<GetLinkResponse>> {
    let table = state.link_table.lock().unwrap();
    let link = table.get(&info.code);
    match link {
        Some(x) => {
            let r = GetLinkResponse {
                link: Some((*x).clone()),
                status_code: 200,
            };
            return Ok(web::Json(r));
        }
        None => {
            let r = GetLinkResponse {
                link: None,
                status_code: 404,
            };
            return Ok(web::Json(r));
        }
    }
}

pub async fn get_local_web_address() -> Result<HttpResponse> {
    match local_ip() {
        Ok(x) => {
            let r = GetLocalIpAddressResponse {
                status_code: 200,
                ip: Some(x.to_string()),
            };
            return Ok(HttpResponse::Ok().json(r));
        }
        Err(_) => {
            let r = GetLocalIpAddressResponse {
                status_code: 404,
                ip: None,
            };
            return Ok(HttpResponse::Ok().json(r));
        }
    };
}

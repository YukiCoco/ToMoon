use actix_web::{body::BoxBody, web, HttpResponse, Result};
use local_ip_address::local_ip;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf, sync::Mutex};

use crate::{
    control::{ClashError, ClashErrorKind},
    helper,
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
}

#[derive(Serialize, Deserialize)]
pub struct GenLinkResponse {
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

pub async fn download_sub(
    state: web::Data<AppState>,
    params: web::Form<GenLinkParams>,
) -> Result<HttpResponse> {
    let url = params.link.clone();
    let runtime = state.runtime.lock().unwrap();

    let runtime_settings;
    let runtime_state;
    unsafe {
        let runtime = runtime.0.as_ref().unwrap();
        runtime_settings = runtime.settings_clone();
        runtime_state = runtime.state_clone();
    }

    let path: PathBuf = PathBuf::from("/home/deck/.config/tomoon/subs");

    //是一个本地文件
    if let Some(local_file) = helper::get_file_path(url.clone()) {
        let local_file = PathBuf::from(local_file);
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
            let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect();
            let path = path.join(s + ".yaml");
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
        match minreq::get(url.clone())
            .with_header(
                "User-Agent",
                format!("ToMoonClash/{}", env!("CARGO_PKG_VERSION")),
            )
            .with_timeout(15)
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
                        return Err(actix_web::Error::from(ClashError {
                            Message: e.to_string(),
                            ErrorKind: ClashErrorKind::InnerError,
                        }));
                    }
                }
                let path = path.to_str().unwrap();
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

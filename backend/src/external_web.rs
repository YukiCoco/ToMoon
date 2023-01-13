use actix_web::{web, HttpResponse, Result};
use local_ip_address::list_afinet_netifas;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

pub struct AppState {
    pub link_table: Mutex<HashMap<u16, String>>,
}

#[derive(Deserialize)]
pub struct GenLinkParams {
    link: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenLinkResponse {
    status_code: u16,
    code: u16,
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

pub async fn gen_link(
    state: web::Data<AppState>,
    params: web::Form<GenLinkParams>,
) -> Result<HttpResponse> {
    let num = rand::thread_rng().gen_range(1000..10000);
    let mut table = state.link_table.lock().unwrap();
    table.insert(num, params.link.clone());
    drop(table);
    let r = GenLinkResponse {
        code: num,
        status_code: 200,
    };
    Ok(HttpResponse::Ok().json(r))
}

pub async fn get_link(
    state: web::Data<AppState>,
    params: web::Form<GetLinkParams>,
) -> Result<web::Json<GetLinkResponse>> {
    let table = state.link_table.lock().unwrap();
    let link = table.get(&params.code);
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
    let network_interfaces = list_afinet_netifas().unwrap();
    for (name, ip) in network_interfaces.iter() {
        if name == "wlan0" { //steamdeck 的网卡名
            let r = GetLocalIpAddressResponse {
                status_code: 200,
                ip: Some(ip.to_string()),
            };
            return Ok(HttpResponse::Ok().json(r));
        }
    }
    let r = GetLocalIpAddressResponse {
        status_code: 404,
        ip: None,
    };
    return Ok(HttpResponse::Ok().json(r));
}

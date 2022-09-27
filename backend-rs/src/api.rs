use std::{process::Command, thread};

use crate::settings;

use super::control::ControlRuntime;

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

mod api;
mod control;
mod external_web;
mod helper;
mod settings;
mod test;

use std::{collections::HashMap, sync::Mutex, thread};

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use simplelog::{LevelFilter, WriteLogger};
use usdpl_back::Instance;

use crate::{
    control::{ControlRuntime, RunningStatus},
    external_web::Runtime,
};

const PORT: u16 = 55555;
const WEB_PORT: u16 = 55556;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    WriteLogger::init(
        #[cfg(debug_assertions)]
        {
            LevelFilter::Debug
        },
        #[cfg(not(debug_assertions))]
        {
            LevelFilter::Info
        },
        Default::default(),
        std::fs::File::create("/tmp/tomoon.log").unwrap(),
    )
    .unwrap();

    log::info!("Starting back-end ({} v{})", api::NAME, api::VERSION);
    log::info!("{}", std::env::current_dir().unwrap().to_str().unwrap());
    println!("Starting back-end ({} v{})", api::NAME, api::VERSION);

    let runtime: ControlRuntime = control::ControlRuntime::new();
    runtime.run();

    let runtime_pr = Runtime(&runtime as *const ControlRuntime);

    thread::spawn(move || {
        Instance::new(PORT)
            .register("set_clash_status", api::set_clash_status(&runtime))
            .register("get_clash_status", api::get_clash_status(&runtime))
            .register("reset_network", api::reset_network())
            .register("download_sub", api::download_sub(&runtime))
            .register("get_download_status", api::get_download_status(&runtime))
            .register("get_sub_list", api::get_sub_list(&runtime))
            .register("get_current_sub", api::get_current_sub(&runtime))
            .register("delete_sub", api::delete_sub(&runtime))
            .register("set_sub", api::set_sub(&runtime))
            .register("update_subs", api::update_subs(&runtime))
            .register("get_update_status", api::get_update_status(&runtime))
            .register("create_debug_log", api::create_debug_log(&runtime))
            .register("get_running_status", api::get_running_status(&runtime))
            .run_blocking()
            .unwrap();
    });

    // 启动一个 tokio 任务来运行 subconverter
    let subconverter_path = helper::get_current_working_dir()
        .unwrap()
        .join("bin/subconverter");
    tokio::spawn(async move {
        if subconverter_path.exists() && subconverter_path.is_file() {
            let mut command = tokio::process::Command::new(subconverter_path);
            // 可以在这里添加命令行参数
            // command.arg("some_argument");

            match command.spawn() {
                Ok(mut child) => {
                    log::info!("Subconverter started with PID: {}", child.id().unwrap());

                    loop {
                        tokio::select! {
                            _ = child.wait() => {
                                log::info!("Subconverter process exited.");
                                break;
                            }
                        }
                    }
                }
                Err(e) => log::error!("Failed to start subconverter: {}", e),
            }
        } else {
            log::error!(
                "Subconverter path does not exist or is not a file: {:?}",
                subconverter_path
            );
        }
    });

    let app_state = web::Data::new(external_web::AppState {
        link_table: Mutex::new(HashMap::new()),
        runtime: Mutex::new(runtime_pr),
    });
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .app_data(app_state.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(
                web::resource("/download_sub").route(web::post().to(external_web::download_sub)),
            )
            .service(web::resource("/get_link").route(web::get().to(external_web::get_link)))
            .service(
                web::resource("/get_ip_address")
                    .route(web::get().to(external_web::get_local_web_address)),
            )
            .service(web::resource("/skip_proxy").route(web::post().to(external_web::skip_proxy)))
            .service(
                web::resource("/override_dns").route(web::post().to(external_web::override_dns)),
            )
            .service(
                web::resource("/enhanced_mode").route(web::post().to(external_web::enhanced_mode)),
            )
            .service(web::resource("/get_config").route(web::get().to(external_web::get_config)))
            //.service(web::resource("/manual").route(web::get().to(external_web.web_download_sub)))
            // allow_remote_access
            .service(
                web::resource("/allow_remote_access")
                    .route(web::post().to(external_web::allow_remote_access)),
            )
            // reload_clash_config
            .service(
                web::resource("/reload_clash_config")
                    .route(web::get().to(external_web::reload_clash_config)),
            )
            // restart_clash
            .service(
                web::resource("/restart_clash").route(web::get().to(external_web::restart_clash)),
            )
            // set_dashboard
            .service(
                web::resource("/set_dashboard").route(web::post().to(external_web::set_dashboard)),
            )
            // web
            .service(
                fs::Files::new("/", "./web")
                    .index_file("index.html")
                    .show_files_listing(),
            )
    })
    .bind(("0.0.0.0", WEB_PORT))
    .unwrap()
    .run()
    .await
}

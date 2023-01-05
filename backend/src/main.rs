mod api;
mod control;
mod helper;
mod settings;
mod test;
mod external_web;

use std::{thread, sync::Mutex, collections::HashMap};

use simplelog::{LevelFilter, WriteLogger};
use usdpl_back::Instance;
use actix_web::{middleware, App, HttpServer, web};
use actix_files as fs;

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

    log::info!(
        "Starting back-end ({} v{})",
        api::NAME,
        api::VERSION
    );
    log::info!("{}", std::env::current_dir().unwrap().to_str().unwrap());
    println!("Starting back-end ({} v{})", api::NAME, api::VERSION);

    let runtime = control::ControlRuntime::new();
    runtime.run();

    thread::spawn(move || {
        Instance::new(PORT)
        .register("set_clash_status", api::set_clash_status(&runtime))
        .register("get_clash_status", api::get_clash_status(&runtime))
        .register("reset_network", api::reset_network())
        .register("download_sub", api::download_sub(&runtime))
        .register("get_download_status", api::get_download_status(&runtime))
        .register("get_sub_list", api::get_sub_list(&runtime))
        .register("delete_sub", api::delete_sub(&runtime))
        .register("set_sub", api::set_sub(&runtime))
        .register("update_subs", api::update_subs(&runtime))
        .register("get_update_status", api::get_update_status(&runtime))
        .register("create_debug_log", api::create_debug_log())
        .register("get_running_status", api::get_running_status(&runtime))
        .run_blocking().unwrap();
    });
    //std::env::set_var("RUST_LOG", "debug");
    //let external_web =  api::ExternalWeb::new(runtime);
    let appState = web::Data::new(external_web::AppState {
        link_table: Mutex::new(HashMap::new()),
    });
     HttpServer::new(move|| {
        App::new()
            .app_data(appState.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/gen_link").route(web::post().to(external_web::gen_link)))
            .service(web::resource("/get_link").route(web::post().to(external_web::get_link)))
            //.service(web::resource("/manual").route(web::get().to(external_web.web_download_sub)))
            .service(fs::Files::new("/", "./web").show_files_listing())
    })
    .bind(("0.0.0.0", WEB_PORT))?
    .run()
    .await
}


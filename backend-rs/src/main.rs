mod api;
mod control;
mod helper;
mod settings;
mod test;

use simplelog::{LevelFilter, WriteLogger};
use usdpl_back::Instance;

const PORT: u16 = 55555;
fn main() -> Result<(), ()> {
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
        std::fs::File::create("/tmp/clashdeck.log").unwrap(),
    )
    .unwrap();

    log::info!(
        "Starting back-end ({} v{} build.12353)",
        api::NAME,
        api::VERSION
    );
    log::info!("{}", std::env::current_dir().unwrap().to_str().unwrap());
    println!("Starting back-end ({} v{})", api::NAME, api::VERSION);

    let runtime = control::ControlRuntime::new();
    runtime.run();

    Instance::new(PORT)
        .register("set_clash_status", api::set_clash_status(&runtime))
        .register("get_clash_status", api::get_clash_status(&runtime))
        .register("reset_network", api::reset_network())
        .register("download_sub", api::download_sub(&runtime))
        .register("get_download_status", api::get_download_status(&runtime))
        .register("get_sub_list", api::get_sub_list(&runtime))
        .register("delete_sub", api::delete_sub(&runtime))
        .run_blocking()
}

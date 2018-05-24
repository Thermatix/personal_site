// crates
extern crate config as config_rs;
extern crate actix;
extern crate actix_web;
extern crate env_logger;

// using crates
use actix_web::{fs, middleware, server, App};

// crate macros
#[macro_use] extern crate log;

// using stdlib
use std::env;

// modules
mod config;

// using modules

use config;

const CONFIG_PREFIX: &str = "WEB";
// log!(level, ...)
// debug!(...)
// info!(...)
// warn!(...)
// error!(...)
fn main() {
    // config keys port, ip, log_level, log_format, rust_backtrace
    let required_keys: [&str; 0] = [];

    let settings =
        match env::args().nth(1) {
        Some(file_path) => config::load(file_path),
        None => config::load("")
    };

    match config::check_config(settings) {
        Some(i) =>  {
            error!( "No `{}` detected in config or env ({}_{}) variables",
                         required_keys[i], CONFIG_PREFIX, required_keys[i].to_uppercase());
            std::process::exit(1);
        },
        None => ()
    };

    log_enabled!(settings.get("log_level").cloned().unwrap_or("info".to_string()));

    ::std::env::set_var("RUST_LOG", settings.get("log_format").cloned().unwrap_or("actix_web=info".to_string()));
    ::std::env::set_var("RUST_BACKTRACE", settings.get("rust_backtrace").cloned().unwrap_or("1".to_string()));

    env_logger::init();

    let sys = actix::System::new("front_end_distro");

    let port = settings.get("port").cloned().unwrap_or("127.0.0.1".to_string());
    let host = settings.get("host").cloned().unwrap_or("3000".to_string());

    server::new(|| {
        App::new()
	        // enable logger
	        .middleware(middleware::Logger::default())
	        .handler(
                "/",
                fs::StaticFiles::new("./public/").index_file("index.html")
            )
    }).bind( format!("{}:{}",settings["host"], settings["port"]))
        .expect("Can not start server on given IP/Port")
        .start();

    info!("Started http server: {}:{}", settings["host"], settings["port"]);
    let _ = sys.run();
}


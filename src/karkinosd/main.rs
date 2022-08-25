mod cli;

use std::env;

macro_rules! os_name {
    (cfg!(target_os = "linux")) => {
        "gnu/linux"
    };
    () => {
        env::consts::OS
    };
}

fn main() {
    let matches = cli::build_cli().get_matches();

    let log_level = match matches.is_present("debug") {
        true => log::Level::Debug,
        false => log::Level::Info,
    };
    simple_logger::init_with_level(log_level).unwrap();

    log::info!(
        "Starting Karkinosd on {} ({})",
        os_name!(),
        env::consts::ARCH
    );
    log::debug!("debug flag !");
}

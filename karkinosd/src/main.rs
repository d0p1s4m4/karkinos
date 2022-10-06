use std::{env, path::PathBuf};

use clap::{ArgGroup, Parser};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

macro_rules! os_name {
    (cfg!(target_os = "linux")) => {
        "gnu/linux"
    };
    () => {
        env::consts::OS
    };
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(ArgGroup::new("network stack").args(["v4", "v6"]),))]
struct Args {
    /// Configuration file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long)]
    interface: Option<Vec<String>>,

    /// Run karkinosd in restricted mode
    #[arg(short, long)]
    restricted: bool,

    /// IPv4 only
    #[arg(short = '4', long)]
    v4: bool,

    /// IPv6 only
    #[arg(short = '6', long)]
    v6: bool,

    /// Set debug
    #[arg(long)]
    debug: bool,

    #[cfg(target_family = "unix")]
    #[arg(short, long)]
    daemonize: bool,

    #[cfg(target_family = "unix")]
    #[arg(short, long, default_value = "/var/run/karkinosd.pid")]
    pid: Option<PathBuf>,
}

fn main() {
    let cli = Args::parse();

    let log_level = match cli.debug {
        true => Level::DEBUG,
        false => Level::TRACE
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("can't setup default subscriber");

    info!(
        "Starting Karkinosd on {} ({})",
        os_name!(),
        env::consts::ARCH
    );
}

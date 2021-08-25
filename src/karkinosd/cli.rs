use clap::{crate_authors, crate_name, crate_version, App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Configuration file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interface")
                .short("I")
                .multiple(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("v4").short("4"))
        .arg(Arg::with_name("v6").short("6"));

    if cfg!(target_family = "unix") {
        cli.arg(
            Arg::with_name("no_daemon")
                .short("D")
                .long("no_deamon")
                .help("Run in foreground"),
        )
    } else {
        cli
    }
}

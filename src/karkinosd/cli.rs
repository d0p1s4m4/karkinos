use clap::{crate_authors, crate_description, crate_version, Arg, Command};

pub fn build_cli() -> Command<'static> {
    let cli = Command::new("karkinosd")
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .help("Configuration file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interface")
                .short('i')
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("restricted")
                .short('r')
                .long("restricted")
                .help("Run Karkinosd in restricted mode"),
        )
        .arg(Arg::with_name("v4").short('4').help("IPv4 only"))
        .arg(
            Arg::with_name("v6")
                .short('6')
                .help("IPv6 only")
                .conflicts_with("v4"),
        )
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("Set debug log level"),
        );
    if cfg!(target_family = "unix") {
        cli.arg(
            Arg::with_name("no_daemon")
                .short('D')
                .long("no-deamon")
                .help("Run in foreground"),
        )
        .arg(
            Arg::with_name("pid")
                .long("pid")
                .takes_value(true)
                .conflicts_with("no_daemon")
                .default_value("/var/run/karkinosd.pid"),
        )
    } else {
        /* assume it's windows */
        cli
    }
}

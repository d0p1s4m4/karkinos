use clap::{crate_authors, crate_description, crate_version, Arg, Command};

pub fn build_cli() -> Command<'static> {
    Command::new("karkinos-cli")
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(
            Arg::with_name("server")
                .short('s')
                .long("server")
                .default_value("127.0.0.1"),
        )
        .arg(Arg::with_name("user").short('u').long("user"))
        .arg(Arg::with_name("password").short('p').long("password"))
        .arg(Arg::with_name("config").short('c').long("config"))
}

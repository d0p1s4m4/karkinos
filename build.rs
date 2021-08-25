use clap::Shell;
use std::env;

pub mod karkinos_cli {
    include!("src/karkinos-cli/cli.rs");
}

pub mod karkinosd {
    include!("src/karkinosd/cli.rs");
}

fn main() {
    let outdir = env::var_os("OUT_DIR").unwrap();

    let mut client = karkinos_cli::build_cli();
    client.gen_completions("karkinos-cli", Shell::Bash, &outdir);
    client.gen_completions("karkinos-cli", Shell::Fish, &outdir);
    client.gen_completions("karkinos-cli", Shell::PowerShell, &outdir);

    let mut deamon = karkinosd::build_cli();
    deamon.gen_completions("karkinosd", Shell::Bash, &outdir);
    deamon.gen_completions("karkinosd", Shell::Fish, &outdir);
    deamon.gen_completions("karkinosd", Shell::PowerShell, &outdir);
}

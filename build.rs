use clap::Command;
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, PowerShell, Zsh},
    Generator,
};
use std::{env, ffi::OsStr};

pub mod karkinos_cli {
    include!("src/karkinos-cli/cli.rs");
}

pub mod karkinosd {
    include!("src/karkinosd/cli.rs");
}

fn completion<G: Generator>(gen: G, cmd: &mut Command, out: &OsStr) {
    generate_to(gen, cmd, cmd.get_name().to_string(), out).expect("Can't generate completion file");
}

fn main() {
    if !cfg!(debug_assertions) {
        let outdir = env::var_os("OUT_DIR").unwrap();
        let mut client = karkinos_cli::build_cli();
        let mut daemon = karkinosd::build_cli();

        if cfg!(target_family = "unix") {
            completion(Bash, &mut client, &outdir);
            completion(Fish, &mut client, &outdir);
            completion(Zsh, &mut client, &outdir);

            completion(Bash, &mut daemon, &outdir);
            completion(Fish, &mut daemon, &outdir);
            completion(Zsh, &mut daemon, &outdir);
        } else {
            /* assume it's windows */
            completion(PowerShell, &mut client, &outdir);
            completion(PowerShell, &mut daemon, &outdir);
        }
    }
}

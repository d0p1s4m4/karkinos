mod cli;

use dirs::config_dir;

fn get_config_file_path() -> String {
    let mut dir = config_dir().unwrap_or(std::path::PathBuf::new());

    dir.push("karkinos");
    dir.push("config");
    dir.set_extension("toml");

    dir.to_str().unwrap_or("").to_string()
}

fn main() {
    let _matches = cli::build_cli().get_matches();

    println!("{}", get_config_file_path());
}

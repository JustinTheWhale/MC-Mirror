extern crate directories;
use directories::BaseDirs;

use std::env;
use std::path::{Path, PathBuf};

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
*/

fn detect_os() -> &'static str {
    return env::consts::OS;
}

fn set_base(os: &str) -> PathBuf {
    if let Some(install_path) = BaseDirs::new() {
        if os == "linux" {
            let mc_base_path = install_path.home_dir().to_path_buf();
            return mc_base_path; 
        } else if os == "windows" {
            let mc_base_path = install_path.preference_dir().to_path_buf();
            return mc_base_path; 
        } else if os == "macos" {
            let mc_base_path = install_path.data_dir().to_path_buf();
            return mc_base_path; 
        }
    };
    return PathBuf::new();
}

fn push_mc_path(base: &mut PathBuf) {
    base.push(".minecraft");
}

fn main() {
    let os: &str = detect_os();
    let mut base: PathBuf = set_base(&os);
    push_mc_path(&mut base);
    let game_path: &Path = base.as_path();

    if game_path.exists() {
        println!("It looks like minecraft is installed here: {:?}", game_path);
    } else {
        println!("It looks like the minecraft folder is installed somwhere other than the defaullt directory or not installed.");
    }
}

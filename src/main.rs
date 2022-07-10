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
    if os == "linux" {
    } else if os == "windows" {
        if let Some(roaming_path) = BaseDirs::new() {
            let mc_base_path = roaming_path.preference_dir().to_path_buf();
            return mc_base_path;
        }
    } else if os == "macos" {
    }
    return PathBuf::new();
}

fn set_mc_path(base: &mut PathBuf) {
    base.push(".minecraft");
}

fn main() {
    let os: &str = detect_os();
    let mut base: PathBuf = set_base(&os);
    set_mc_path(&mut base);
    let game_path: &Path = base.as_path();

    if game_path.exists() {
        println!("It looks like minecraft is installed here: {:?}", game_path);
    } else {
        println!("It looks like the minecraft folder is installed somwhere other than the defaullt directory or not installed.");
    }
}

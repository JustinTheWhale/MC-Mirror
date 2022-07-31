extern crate directories;
extern crate serde_json;
use directories::BaseDirs;

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use serde_json::Value;

fn detect_os() -> &'static str {
    return env::consts::OS;
}

fn set_base(os: &str) -> PathBuf {
    if let Some(install_path) = BaseDirs::new() {
        if os == "linux" {
            let mc_base_path: PathBuf = install_path.home_dir().to_path_buf();
            return mc_base_path;
        } else if os == "windows" {
            let mc_base_path: PathBuf = install_path.preference_dir().to_path_buf();
            return mc_base_path;
        } else if os == "macos" {
            let mc_base_path: PathBuf = install_path.data_dir().to_path_buf();
            return mc_base_path;
        }
    };
    return PathBuf::new();
}

fn push_mc_path(base: &mut PathBuf) {
    base.push(".minecraft");
}

fn config_check() -> bool {
    return Path::new("mirrorconfig.json").exists();
}

fn create_mirrorconfig() {
    let _f = fs::File::create("mirrorconfig.json");
    let _f = match _f {
        Ok(file) => file,
        Err(error) => panic!("Problem creating mirrorconfig: {:?}", error),
    };
}

fn delete_mirrorconfig() {
    assert!(config_check());
    fs::remove_file("mirrorconfig.json").expect("Couldn't delete mirrorconfig for some reason");
}

fn write_mirrorconfig(config: Value) {
    let _f = fs::write("mirrorconfig.json", config.to_string());
    let _f = match _f {
        Ok(file) => file,
        Err(error) => panic!("Problem writing to mirrorconfig: {:?}", error),
    };
}

fn is_number(selection: String) -> bool {
    for c in selection.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn verify_selection(user_input: &String, max_worlds: usize) -> Vec<u8> {
    let mut selections: Vec<u8> = Vec::new();
    let max_possible: u8 = max_worlds as u8;

    //Check if only one world was selected
    if user_input.chars().count() == 1 || !user_input.contains(",") {
        if is_number(user_input.to_string()) {
            let num: u8 = user_input.parse().unwrap();

            if num <= max_possible && num != 0 {
                selections.push(num);
            } else {
                delete_mirrorconfig();
                panic!("Invalid input");
            }
        } else {
            delete_mirrorconfig();
            panic!("Invalid input");
        }
    } else if user_input.contains(",") {
        let worlds: Vec<&str> = user_input.split(",").collect();
        for selection in &worlds {
            if is_number(selection.to_string()) {
                let num: u8 = selection.parse().unwrap();
                if num <= max_possible && num != 0 {
                    selections.push(num);
                } else {
                    delete_mirrorconfig();
                    panic!("Invalid input");
                }
            } else {
                delete_mirrorconfig();
                panic!("Invalid input");
            }
        }
    }
    return selections;
}

fn confirm_worlds(worlds_path: &mut PathBuf) -> Vec<PathBuf> {
    let mut worlds: Vec<PathBuf> = Vec::new();
    let mut saves: Vec<PathBuf> = Vec::new();
    let mut input: String = String::new();

    for world in fs::read_dir(worlds_path).unwrap() {
        worlds.push(world.unwrap().path())
    }

    println!("\n");
    println!("{}", "Please enter the number(s)corresponding to which worlds you want to sync, separated by a comma. \nFor example, if you want worlds 1 2 and 3, type '1,2,3' - If you only want to sync one world, just enter a single number.");

    for i in 0..worlds.len() {
        let save: PathBuf = worlds[i]
            .iter()
            .skip_while(|ending| *ending != "saves")
            .skip(1)
            .collect();
        println!("{}). {}", i + 1, save.display());
    }

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get user input!");
    // Removes line fracture and newline
    input.truncate(input.len() - 2);

    let valid: Vec<u8> = verify_selection(&input, worlds.len());

    for i in 0..valid.len() {
        let index: usize = (valid[i] - 1) as usize;
        let x: PathBuf = (&worlds[index]).to_path_buf();
        saves.push(x)
    }

    return saves;
}

fn first_time_setup() {
    //First time setup
    let os: &str = detect_os();
    let mut base: PathBuf = set_base(&os);
    assert!(
        base.to_str() != Some(""),
        "Unable to determine OS or unsupported OS, exiting..."
    );

    push_mc_path(&mut base);
    let mut game_path: PathBuf = base;

    if game_path.exists() {
        println!("It looks like minecraft is installed here: {:?}", game_path);
    } else {
        println!("It looks like the minecraft folder is installed somwhere other than the defaullt directory or not installed.");
    }

    create_mirrorconfig();
    assert!(
        config_check(),
        "Couldn't create a config file for some reason, exiting..."
    );

    //Looking for MC worlds
    let saves_path: &mut PathBuf = &mut game_path;
    saves_path.push("saves");
    let saves: Vec<PathBuf> = confirm_worlds(saves_path);

    let config = serde_json::json!({
        "config": {
            "operating_system": os,
            "game_path": game_path.to_str(),
            "timestamp": "false",
            "saves" : saves,
        }
    });

    write_mirrorconfig(config);
}

fn main() {
    let now = Instant::now();
    if config_check() == false {
        first_time_setup();
    } else {
        first_time_setup();
    }
}

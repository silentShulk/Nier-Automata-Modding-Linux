use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io::BufReader;
use std::io::stdin;
use std::path::PathBuf;
use std::process::Command;
use std::{collections::hash_map::HashMap};
//use clap::{Arg, ArgAction}; // Will be used to add arguments
use clap::Parser;
use serde::{Deserialize, Serialize};

mod mod_managing;
use crate::mod_managing::features::*;



// Path for the json file containing data on game's path and installed mods
const DATA_FILE_PATH: &str = "~/.config/NAHML/data.json";

// The various types of mod that can be installed with ATA
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ModType {
    Textures,
    PlayerModels,
    WeaponModels,
    WorldModels,
    CutsceneReplacements,
    ReshadePreset
}

// Things to take note about a mod for both mod managing and informing the user
#[derive(Serialize, Deserialize)]
struct Mod {
    name: String,
    files: Vec<PathBuf>,
    enabled: bool,
    mod_type: ModType,
}

// What to save in the data file
#[derive(Serialize, Deserialize)]
struct Config {
    game_path: PathBuf,
    mods: Vec<Mod>,
}
impl Config {
    // Save the config to file
    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data_file = File::create(DATA_FILE_PATH).expect("Failed to open data file (~/.config/NAMHL/data.json)");
        serde_json::to_writer_pretty(data_file, self)?;
        Ok(())
    }

    // Load the config from file
    fn load_config() -> Option<Result<Self, Box<dyn std::error::Error>>> 
    // Returns Some if the file exists, None if it doesn't
    // If the file exists it, the Some can contain
    // Ok(config), Err(error generated while trying to access the file)
    {
        if PathBuf::from(DATA_FILE_PATH).exists() {
            let data_file = File::open(DATA_FILE_PATH);
            match data_file {
                Ok(data) => {
                    let reader = BufReader::new(data);
                    let contents = serde_json::from_reader(reader);
                    match contents {
                        Ok(config) => Some(Ok(config)),
                        Err(er) => return Some(Err(Box::new(er)))
                    }
                }
                Err(er) => return Some(Err(Box::new(er)))
            }
        }
        else {
            None
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self { game_path: PathBuf::from("~/.local/share/Steam/steamapps/common/NieRAutomata"), mods: Default::default() }
    }
}



fn main() {
    println!("WELCOME TO THE NIER AUTOMATA MOD HELPER for LINUX (NAMHL)");

    /* ------------------- */
    /*   STARTING CHECKS   */
    /* ------------------- */

    // LOAD DATA IF PRESENT
    let mut current_config = Config::load_config().unwrap_or_else(|| {
        println!("No data file found (maybe it's the first time you use this program?), defaulting to default game path...");
        Ok(Config::default()) 
    }).expect("Failed to load config file (~/.config/NAHML/data.json)");

    // CHECKING GAME PATH LOCATION
    if !current_config.game_path.exists() {
        correct_gamepath(&mut current_config.game_path);
    } else {
        println!("Game installation found at {:?}", current_config.game_path)
    }

    // CHECKING IF REQUIRED MODDING FILES ARE INSTALLED
    check_for_required_modding_files(&current_config.game_path);

    /* -------------------- */
    /*   USER INTERACTION   */
    /* -------------------- */

    let mut action_id = String::from("");

    while action_id != "0" {
        ask_user_action(&mut action_id);
    }
}



/* ---------- */
/*   CHECKS   */
/* ---------- */

// CHECKING GAME PATH LOCATION
fn correct_gamepath(game_path: &mut PathBuf) {
    println!(
        "Game installation not found at: $HOME/.local/share/Steam/steamapps/common/NieRAutomata"
    );
    println!("Insert your game path: ");

    let mut new_path = String::new();
    stdin()
        .read_line(&mut new_path)
        .expect("Failed to read input");

    *game_path = PathBuf::from(new_path);
}

// CHECKING IF REQUIRED MODDING FILES ARE ALREADY PRESENT
fn check_for_required_modding_files(game_path: &PathBuf) {
    let game_files = read_dir(game_path) // Get the files in the game's directory
        .expect("Failed to read contents of game's directory");

    // List of required modding files
    let mut required_modding_files_needed = HashMap::from([
        (
            PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata(original).exe",),    // Original NieRAutomata exe with name changed according to my standards
            true,
        ), 
        (
            PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata.exe",),      // Modded game exe, both original and modded need to be present
            true,
        ),
        (
            PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/d3d11.dll"),      // SpecialK dll
            true,
        ), 
    ]);

    // Check which files are present in the game's directory
    for entry in game_files {
        let entry_path = entry.expect("Couldn't read one of the files").path();
        if required_modding_files_needed.contains_key(&entry_path) {
            required_modding_files_needed
                .entry(entry_path)
                .and_modify(|val| *val = !*val);
        }
    }

    let modding_files_present = required_modding_files_needed
        .values()
        .all(|&val| val == false);
    match modding_files_present {
        true => println!("Required modding files already installed. That's good"), // If files are already installed
        false => missing_files_warning(),
    }
}

// IF MODDING FILES AREN'T PRESENT, WARN THE USER
fn missing_files_warning() {
    println!(
        "Required modding files are missing, you need to install them if you want to mod the game"
    );
    println!("Start installation of required modding files? [Y/n]");
    let mut answer = String::new();
    match stdin().read_line(&mut answer) {
        Ok(_) => {
            if answer == "Y" || answer == "" {
                run_auto_install_script();
            } // Running the script to install the required modding files
        }
        Err(er) => eprintln!("Couldn't read answer, {er}"),
    }
}

// IF THE USER WANTS TO, INSTALL THE FILES
fn run_auto_install_script() {
    let exe_path = env::args_os()
        .next()
        .expect("Couldn't locate program's path");
    let mut script_path = PathBuf::from(exe_path);
    script_path.pop();
    script_path.push("install_prerequisites.sh");

    let status = Command::new("bash").arg(&script_path).status();

    match status {
        Ok(_) => println!("File executed succesfully, required modding files installed"),
        Err(er) => eprintln!("Failed to execute files {}", er),
    }
}

/* ---------------------- */
/*   USER ACTION CHOICE   */
/* ---------------------- */

fn ask_user_action(action_id: &mut String) {
    // Asking what the user wants to do
    println!(
        "What do you want to do?\n
            \t1 - Install a mod (you have to provide a zip folder of the mod)
            \t2 - Uninstall a mod (you have to type the name of the mod)
            \t0 - Close the NAMHL"
    );
    println!("\nInsert a number");

    // Getting the user's action's id
    stdin().read_line(action_id).expect("Failed to read answer");

    // Starting one of the features
    match action_id.trim() {
        "1" => install_mod(),
        "2" => uninstall_mod(),
        "0" => println!("Happy Automata"),
        _ => println!("{action_id} is not an action id (input either 1, 2, 3 or 0)"),
    }
}



/* ---------------------------- */
/*   FLAGS FOR QUICK FEATURES   */
/* ---------------------------- */

#[derive(Parser)]
#[command(
    name = "NAMHL",
    version = "0.01",
    about = "The Nier Automata Mod Helper for Linux"
)]
struct Args {
    folder_path: String,
    mod_name: String,
    // Will add arguments here
}

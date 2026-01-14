use std::env;
use std::collections::hash_map::HashMap;
use std::io::stdin;
use std::path::PathBuf;
use std::fs::read_dir;
use std::process::Command;
use clap::{Arg, ArgAction}; // Will be used to add arguments 
use clap::Parser;

use crate::features::{install_mod, monologue, uninstall_mod}; // Used for getting the argument of the folder path
mod features;



fn main() {
    println!("WELCOME TO THE NIER AUTOMATA MOD HELPER for LINUX (NAMHL)");



    /* ------------------- */
    /*   STARTING CHECKS   */
    /* ------------------- */

    let mut game_path = PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata");    // Assume default game path

    // CHECKING GAME PATH LOCATION
    check_gamepath(&mut game_path);

    // CHECKING IF REQUIRED MODDING FILES ARE INSTALLED
    check_for_required_modding_files(&game_path);



    /* -------------------- */
    /*   USER INTERACTION   */
    /* -------------------- */

    ask_user_action();
}






/* ---------- */
/*   CHECKS   */
/* ---------- */

// CHECKING GAME PATH LOCATION
fn check_gamepath(game_path: &mut PathBuf) {
    // If the default path doesn't exist, ask the user for the correct path
    if !game_path.exists() {
        println!("Game installation not found at: $HOME/.local/share/Steam/steamapps/common/NieRAutomata");
        println!("Insert your game path: ");

        let mut new_path = String::new();
        stdin()
            .read_line(&mut new_path)
            .expect("Failed to read input");

        *game_path = PathBuf::from(new_path);
    }
    else {
        println!("Game installation found at {:?}", game_path)
    }
}

// CHECKING IF REQUIRED MODDING FILES ARE ALREADY PRESENT
fn check_for_required_modding_files(game_path: &PathBuf) {
    let game_files = read_dir(game_path)    // Get the files in the game's directory
        .expect("Failed to read contents of game's directory");

    // List of required modding files
    let mut required_modding_files_needed = HashMap::from([
        (PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata(original).exe"), true),     // Original NieRAutomata exe with name changed according to my standards
        (PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata.exe"), true),               // Modded game exe, both original and modded need to be present
        (PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/d3d11.dll"), true)                       // SpecialK dll
    ]);

    // Check which files are present in the game's directory
    for entry in game_files {
        let entry_path = entry.expect("Couldn't read one of the files").path();
        if required_modding_files_needed.contains_key(&entry_path) {
            required_modding_files_needed.entry(entry_path)
                .and_modify(|val| *val = !*val);
        }
    }

    let modding_files_present = required_modding_files_needed.values().all(|&val| val == false);
    match modding_files_present {
        true => println!("Required modding files already installed. That's good"),    // If files are already installed
        false => missing_files_warning()
    }
}

// IF MODDING FILES AREN'T PRESENT WARN THE USER
fn missing_files_warning() {
    println!("Required modding files are missing, you need to install them if you want to mod the game");
    println!("Start installation of required modding files? [Y/n]");
    let mut answer = String::new();
    match stdin().read_line(&mut answer) {
        Ok(_) => {
            if answer == "Y" || answer == "" { run_auto_install_script(); }    // Running the script to install the required modding files
        }
        Err(er) => eprintln!("Couldn't read answer, {er}")
    }
}

// RUNNING INSTALL SCRIPT
fn run_auto_install_script() {
    let exe_path = env::args_os().next()
        .expect("Couldn't locate program's path");
    let mut script_path = PathBuf::from(exe_path);
    script_path.pop();
    script_path.push("install_prerequisites.sh");

    let status = Command::new("bash") 
        .arg(&script_path)
        .status();

    match status {
        Ok(_) => println!("File executed succesfully, required modding files installed"),
        Err(er) => eprintln!("Failed to execute files {}", er)
    }
}



/* ---------------------- */
/*   USER ACTION CHOICE   */
/* ---------------------- */

fn ask_user_action() {
    let mut action_id = String::from("");

    while action_id != "0" {
        action_id = String::from("");
        // Asking what the user wants to do 
        println!("What do you want to do?\n
                    \t1 - Install a mod (you have to provide a zip folder of the mod)
                    \t2 - Uninstall a mod (you have to type the name of the mod)
                    \t3 - Read 2B's monologue (first line of the game)
                    \t0 - Close the NAMHL");
        println!("\nInsert a number");

        // Getting the user's action's id
        stdin()
            .read_line(&mut action_id)
            .expect("Failed to read input");



        // Starting one of the features
        match action_id.trim() {
            "1" => install_mod(),
            "2" => uninstall_mod(),
            "3" => monologue(),
            "0" => println!("Happy Automata"),
            _ => println!("{action_id} is not an action id (input either 1, 2 or 3)"),
        }
    }
}










/* ---------------------------- */
/*   FLAGS FOR QUICK FEATURES   */
/* ---------------------------- */

#[derive(Parser)]
#[command(name = "NAMHL", version = "0.01", about = "The Nier Automata Mod Helper for Linux")]
struct Args {
    folder_path: String,
    mod_name: String,

    // Will add arguments here
}

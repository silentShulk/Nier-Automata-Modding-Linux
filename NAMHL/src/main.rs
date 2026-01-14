use std::env;
use std::collections::hash_map::{self, HashMap};
use std::io::{self, Read, stdin};
use std::path::{Path, PathBuf};
use std::fs::{read, read_dir};
use std::process::{Command, ExitStatus}; use clap::builder::OsStr;
// Will be used for checking files in mod zip file
use clap::{Arg, ArgAction}; // Will be used to add arguments 
use clap::Parser; // Used for getting the argument of the folder path



fn main() {
    println!("WELCOME TO THE NIER AUTOMATA MOD HELPER for LINUX (NAMHL)");



    /* --------------- */
    /* STARTING CHECKS */
    /* --------------- */
    let mut game_path = PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata");    // Assume default game path

    check_gamepath(&mut game_path);




    // CHECKING IF REQUIRED MODDING FILES ARE INSTALLED
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

    // Printing whether the required modding files are already installed
    let modding_files_present = required_modding_files_needed.values().all(|&val| val == false);
    match modding_files_present {
        true => println!("Required modding files already installed. That's good"),    // If files are already installed
        false => {    // If files aren't already installed
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
    }



    /* ---------------- */
    /* USER INTERACTION */
    /* ---------------- */

    let mut action_id = String::from("");

    while action_id != "0" {
        action_id = String::from("");
        // WELCOMING THE USER 
        println!("What do you want to do?\n
                    \t1 - Install a mod (you have to provide a zip folder of the mod)
                    \t2 - Uninstall a mod (you have to type the name of the mod)
                    \t3 - Read 2B's monologue (first line of the game)
                    \t0 - Close the NAMHL");
        print!("\nInsert a number");

        // GETTING THE USER'S ACTION'S ID
        stdin()
            .read_line(&mut action_id)
            .expect("Failed to read input");



        // STARTING ONE OF THE FEATURES
        match action_id.trim() {
            "1" => install_mod(),
            "2" => uninstall_mod(),
            "3" => monologue(),
            "0" => println!("Happy Automata"),
            _ => println!("{action_id} is not an action id (input either 1, 2 or 3)"),
        }
    }
}



/* ----------------------------------- */
/* CHECKING FOR GAME INSTALLATION PATH */
/* ----------------------------------- */
fn check_gamepath(game_path: &mut PathBuf) {
    // If the default path doesn't exist, ask the user for the correct path
    if !game_path.exists() {
        println!("Game installation not found at: $HOME/.local/share/Steam/steamapps/common/NieRAutomata");
        print!("Insert your game path: ");

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


/* ---------------------- */
/* RUNNING INSTALL SCRIPT */
/* ---------------------- */

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



/* -------------------- */
/*   MOD INSTALLATION   */
/* -------------------- */

fn install_mod() {
    println!("FEATURE NOT YET IMPLEMENTED");
}



/* ------------------ */
/* MOD UNINSTALLATION */
/* ------------------ */

fn uninstall_mod() {
    println!("FEATURE NOT YET IMPLEMENTED");
}



/* -------------- */
/* 2B'S MONOLOGUE */
/* -------------- */

fn monologue() {
    println!("Everything that lives is designed to end,\n
            we're perpetually trapped... In a never ending cycle of life and death.
            I often think about the god who blessed us with this cryptic puzzle,
            and wonder if we'll ever have the chance to kill him");
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

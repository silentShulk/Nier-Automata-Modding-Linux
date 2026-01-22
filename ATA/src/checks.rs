use std::collections::hash_map::HashMap;
use std::env;
use std::env::current_exe;
use std::fs::File;
use std::fs::read_dir;
use std::io::Write;
use std::io::{BufReader, stdin, stdout};
use std::path::PathBuf;
use std::process::Command;
use std::error::Error;
use std::process::ExitStatus;
//use clap::{Arg, ArgAction}; // Will be used to add arguments
use clap::Parser;
use serde::{Deserialize, Serialize};


/* ---------- */
/*   CHECKS   */
/* ---------- */

// CHECK IF GIVEN PATH CONTAINS GAME FILES
fn check_path(current_path: &PathBuf) -> Result<bool, std::io::Error> {
    let is_gamepath = read_dir(current_path)?       
        .filter_map(|res| {     // For each entry return Some(is-exe) or None and warn the user that an entry couldn't be read
            match res {
                Ok(entry) => Some(entry.file_name() == "NieRAutomata.exe"),
                Err(e) => {
                    eprintln!("Warning: Could not read an entry in the given path: {}", e);
                    None 
                }
            }
        })
        .any(|is_match| is_match);      // Check if any of the entries that were read matched the is-exe predicate

    Ok(is_gamepath)
}

// CHECKING GAME PATH LOCATION
fn ask_for_correct_gamepath(wrong_path: &mut PathBuf) -> Result<PathBuf, std::io::Error> {
    println!(
        "Game installation not found at: {:?}", wrong_path
    );
    print!("Insert your game path: ");
    stdout().flush()?;

    let mut new_path = String::new();
    stdin().read_line(&mut new_path)?;

    Ok(PathBuf::from(new_path.trim()))
}

// CHECKING IF REQUIRED MODDING FILES ARE ALREADY PRESENT
fn check_for_required_modding_files(game_path: &PathBuf)-> Result<bool, Box<dyn Error>> {
    let game_files = read_dir(game_path)?; // Get the files in the game's directory

    // List of required modding files
    let mut required_modding_files_needed = HashMap::from([
        (
            PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata(original).exe",), // Original NieRAutomata exe with name changed according to my standards
            true,
        ),
        (
            PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata.exe",), // Modded game exe, both original and modded need to be present
            true,
        ),
        (
            PathBuf::from("$HOME/.local/share/Steam/steamapps/common/NieRAutomata/d3d11.dll"), // SpecialK dll
            true,
        ),
    ]);

    // Check which of the required modding files are present in the game's directory
    for entry in game_files {
        let entry_path = entry?.path();
        if required_modding_files_needed.contains_key(&entry_path) {
            required_modding_files_needed
                .entry(entry_path)
                .and_modify(|val| *val = !*val);
        }
    }

    let modding_files_present = required_modding_files_needed
        .values()
        .all(|&val| val == false);

    Ok(modding_files_present)
}

// IF MODDING FILES AREN'T PRESENT, WARN THE USER
fn missing_files_warning() -> Result<bool, std::io::Error> {
    println!(
        "Required modding files are missing, you need to install them if you want to mod the game"
    );
    print!("Start installation of required modding files? [Y/n]");
    stdout().flush()?;

    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    
    if answer.trim() == "Y" || answer.trim() == "y"  || answer.trim() == "" {
        Ok(true)
    } else {
        Ok(false)
    }
}

// IF THE USER WANTS TO, INSTALL THE FILES
fn run_auto_install_script() -> Result<ExitStatus, Box<dyn Error>> {
    let script_path = PathBuf::from("/usr/local/bin/ATA/install-prerequisites.sh");

    print!("Insert the name of terminal emulator:");
    stdout().flush()?;
    let mut answer = String::new();
    stdin().read_line(&mut answer)?;

    let status = Command::new(answer)
        .arg("-e")
        .arg("bash")
        .arg(&script_path)
        .status();

    match status {
        Ok(exit_status) => Ok(exit_status),
        Err(er) => Err(Box::new(er))
    }
}
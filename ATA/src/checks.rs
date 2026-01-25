use std::collections::hash_map::HashMap;
use std::fs::read_dir;
use std::io::Write;
use std::io::{stdin, stdout};
use std::path::PathBuf;
use std::process::Command;
use std::error::Error;
use std::process::ExitStatus;
//use clap::{Arg, ArgAction}; // Will be used to add arguments


/* ---------- */
/*   CHECKS   */
/* ---------- */

// CHECK IF GIVEN PATH CONTAINS GAME FILES
pub fn check_path(current_path: &PathBuf) -> Result<bool, std::io::Error> {
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
pub fn ask_for_correct_gamepath(wrong_path: &mut PathBuf) -> Result<PathBuf, std::io::Error> {
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
pub fn check_for_required_modding_files(game_path: &PathBuf) -> Vec<PathBuf> {
    let required_files = [
        "NieRAutomata.exe",
        "d3d11.dll",
    ];

    let missing_files: Vec<PathBuf> = required_files
        .iter()
        .map(|&file| game_path.join(file))
        .filter(|path| !path.exists())
        .collect();
    
    missing_files
}

// IF MODDING FILES AREN'T PRESENT, WARN THE USER
pub fn missing_files_warning(missing_files: Vec<PathBuf>) -> Result<bool, std::io::Error> {
	for missing_file in missing_files {
		println!("{:?} is missing", missing_file)
	}
    println!("You need to install this files if you want to mod the game");
    
    print!("Start installation of required modding files? [Y/n]");
    stdout().flush()?;

    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    let answer = answer.trim();
    
    if answer.is_empty() || answer.eq_ignore_ascii_case("y") {
        Ok(true)
    } else {
        Ok(false)
    }
}

// IF THE USER WANTS TO, INSTALL THE FILES
pub fn run_auto_install_script() -> Result<ExitStatus, Box<dyn Error>> {
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
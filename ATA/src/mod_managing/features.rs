use std::{self, io::Write};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::error::Error;
use zip::ZipArchive;
use std::io::{stdin, stdout};
use walkdir::WalkDir;

use super::installation_methods::*;
use crate::*;



/* -------------------- */
/*   MOD INSTALLATION   */
/* -------------------- */

pub fn install_mod(game_path: &PathBuf) -> Result<Mod, Box<dyn Error>> {
    // Ask the user for mod path
    println!("To install a mod type the path to the compressed folder of a mod you downloaded\n\
            IT HAS TO BE A COMPRESSED FOLDER (.zip, .7z, .rar)");
    print!("Insert path >> ");
    stdout().flush()?;
    
    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    let answered_path = PathBuf::from(answer.trim());
    
    // Check if it exists
    if !answered_path.exists() {
        return Err("Mod path does not exist".into());
    }
    
    // Unzip the mod folder
    let mut mod_folder_path = unzip_folder(&answered_path)?;
    
    // Get the type of mod containd
    let mod_data = check_mod_type(&mut mod_folder_path)?
       	.ok_or("The given path doesn't contain a mod")?;
    // Install the mod contained in the folder following the correct installation method
    let installed_mod = match mod_data.0 {
       	ModType::Textures => install_texture(mod_folder_path, game_path)?,
       	ModType::PlayerModels => install_player_model(mod_folder_path, game_path)?,
       	ModType::WeaponModels => install_weapon_model(mod_folder_path, game_path)?,
       	ModType::WorldModels => install_world_model(mod_folder_path, game_path)?,
        ModType::CutsceneReplacements => install_cutscene_replacements(mod_folder_path, game_path)?,
        ModType::ReshadePreset => install_reshade_preset(mod_folder_path, game_path)?,
    };
    
    Ok(installed_mod)
}



/* ---------------------- */
/*   MOD UNINSTALLATION   */
/* ---------------------- */

pub fn uninstall_mod(game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}



/* ------------------------ */
/*   MOD TYPE RECOGNITION   */
/* ------------------------ */

fn check_mod_type(mod_folder_path: &mut PathBuf) -> Result<Option<(ModType, PathBuf)>, Box<dyn Error>> {
    // Define variables that will be returned
    let mut mod_files_path: Option<PathBuf> = None;
    let mut mod_contained: Option<ModType> = None;
    
    // Start looking at the contents of mod folder
    for entry in WalkDir::new(&mod_folder_path) {
        let current_entry = entry?;
        let entry_path = current_entry.path();
        
        // Skip folders
        if !current_entry.file_type().is_file() {
           	continue
        }         
        // Get current entry file extension
        let extension = match get_file_extension(entry_path) {
            Ok(ext) => ext,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        // For each valid entry check if it is the file of a mod
        mod_contained = match extension {
            "dss" => Some(ModType::Textures),
            "dtt" | "dat" => {
                let Some(name) = entry_path.file_name() else {
                    println!("\"{:?}\" is a path that ends in .. (parent directory) or . (current directory), and will therefore be skipped", entry_path);
                    continue;
                };
                match name.to_str() {
                    Some("pl") => Some(ModType::PlayerModels),
                    Some("wp") => Some(ModType::WeaponModels),
                    Some("bg") => Some(ModType::WorldModels),
                    Some(_) => None,
                    None => {
                        println!("\"{:?}\" contains invalid Unicode in its name and will therefore will be skipped", entry_path);
                        continue;
                    }
                }
            }  // RESHADE
            "usm" => Some(ModType::CutsceneReplacements),
            _ => None,
        };

        if mod_contained.is_some() {
            // Update mod_files_path
            mod_files_path = Some(entry_path.to_path_buf());
            break;
        }
    }
    
    Ok(mod_contained.zip(mod_files_path))
}
    
fn unzip_folder(zipped_mod_folder: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let mod_file = File::open(zipped_mod_folder)?;
    let mut mod_zip_archive = ZipArchive::new(mod_file)?;
    let extraction_target_folder = zipped_mod_folder
    	.parent()
     	.ok_or("Cannot find parent directory")?;
    
    mod_zip_archive.extract(&extraction_target_folder)?;
    Ok(extraction_target_folder.to_path_buf())
}

fn get_file_extension(entry_path: &Path) -> Result<&str, String> {
    let Some(extension) = entry_path.extension() else {
        return Err(String::from(format!("{:?} is an extensionless file, therefore it will be skipped", entry_path)));
    };
    let Some(extension_str) = extension.to_str() else {
       	return Err(String::from(format!("{:?} contains invalid UTF-8 in its extension, therefore it will be skipped", entry_path)));
    };
    
    Ok(extension_str)
}


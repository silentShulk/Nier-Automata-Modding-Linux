use std::{self, io::Write};
use std::fs::File;
use std::path::PathBuf;
use std::error::Error;
use zip::ZipArchive;
use std::io::{stdin, stdout};
use walkdir::WalkDir;

use super::installation_methods::*;
use crate::*;



/* -------------------- */
/*   MOD INSTALLATION   */
/* -------------------- */

pub fn install_mod(game_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    println!("To install a mod type the path to the compressed folder of a mod you downloaded\n\
            IT HAS TO BE A COMPRESSED FOLDER (.zip, .7z, .rar)");
    print!("Insert path >> ");
    stdout().flush()?;
    
    let mut answer = String::new();
    stdin().read_line(&mut answer)?;
    let answered_path = PathBuf::from(answer.trim());
    
    if answered_path.exists() {
        let (mod_type, mod_files_path) = check_mod_type(answered_path)
        	.ok_or("The given path doesn't contain a mod")?;
        match mod_type {
           	ModType::Textures => {
                let installed_mod = install_texture(mod_files_path, game_path)?;
                save_mod_data(installed_mod);
            }
           	ModType::PlayerModels => {
                let installed_mod = install_player_model(mod_files_path, game_path)?;
                save_mod_data(installed_mod);
            }
           	ModType::WeaponModels => {
            	let installed_mod = install_weapon_model(mod_files_path, game_path)?;
            	save_mod_data(installed_mod);
            }
           	ModType::WorldModels => {
            	let installed_mod = install_world_model(mod_files_path, game_path)?;
            	save_mod_data(installed_mod);
            }
            ModType::CutsceneReplacements => {
            	let installed_mod = install_cutscene_replacements(mod_files_path, game_path)?;
            	save_mod_data(installed_mod);
            }
            ModType::ReshadePreset => {
            	let installed_mod = install_reshade_preset(mod_files_path, game_path)?;
            	save_mod_data(installed_mod);
            }
        }
        
        Ok(())
    } 
    else {
        Err("Mod path does not exist".into())
    }
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

fn check_mod_type(zip_path: PathBuf) -> Result<Option<(ModType, PathBuf)>, Box<dyn Error>> {
    let mod_folder_path = unzip_folder(&zip_path)?;
    
    for entry in WalkDir::new(&mod_folder_path) {
        let current_entry = entry?;
        let entry_path = current_entry.path();
        
        if !current_entry.file_type().is_file() {
           	continue
        } 
        let Some(extension) = entry_path.extension() else {
            println!("{:?} is an extensionless file, therefore it will be skipped", entry_path);
            continue;
        };
        let Some(ext_str) = extension.to_str() else {
       		println!("{:?} contains invalid UTF-8 in its name, therefore it will be skipped", entry_path);
         	continue;
        };
              
        let mod_type = match ext_str {
            "dss" => ModType::Textures,
            "dtt" | "dat" => {
                let name = entry_path.file_name();
                match name {
                    Some("pl") => ModType::PlayerModels,
                    "wp" => ModType::WeaponModels,
                    "bg" => ModType::WorldModels,
                    _ => continue,
                }
            }
            "usm" => ModType::CutsceneReplacements,
            _ => ()
        }; // RESHADE
    }
    
    Ok(None)
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

fn save_mod_data(mod_data: Mod) -> Result<(), Box<dyn Error>> {
	Ok(())
}
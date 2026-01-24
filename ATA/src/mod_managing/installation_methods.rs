use std::fs::read_dir;
use std::path::PathBuf;
use std::io::{Write, stdin, stdout};
use std::error::Error;

use crate::*;

pub fn install_texture(dss_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn Error>> {
	let mod_name = ask_mod_name()?;
    let dss_files = read_dir(dss_folder_path).unwrap();
    let textures_folder = PathBuf::from("");
    
    for file in dss_files {
        let entry = file?;
        let test_path = entry.path();
    }
    
    Ok(Mod::new(mod_name, vec![], true, ModType::Textures))
}

pub fn install_player_model(dtt_dat_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>>  {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}

pub fn install_weapon_model(dtt_dat_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}

pub fn install_world_model(dtt_dat_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}

pub fn install_cutscene_replacements(usm_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}

pub fn install_reshade_preset(preset_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}



/* -------------------------- */
/*   INSTALLATION FUNCTIONS   */
/* -------------------------- */

fn ask_mod_name() -> Result<String, std::io::Error> {
	print!("Insert name of the mod that you are installing (choose anything you want, will be used as identifier)");
	stdout().flush()?;
	
	let mut answer = String::new();
	stdin().read_line(&mut answer)?;
	Ok(answer)
}


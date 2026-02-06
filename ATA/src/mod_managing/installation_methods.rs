use std::fs::read_dir;
use std::path::PathBuf;
use std::io::{Write, stdin, stdout};
use std::error::Error;
use std::fs::copy;

use crate::*;

pub fn install_texture(dss_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn Error>> {
    println!("Give a name to the mod being installed");
    print!("Name: ");
    stdout().flush()?;
    let mut answer = String::new();
    stdin().read_line(&mut answer)?;

    let texture_mods_folder = game_path.join("/SK_Res/inject/textures");

    let mut mod_files: Vec<PathBuf> = vec![];
    for entry in read_dir(dss_folder_path)? {
        let current_entry = entry?;
        let entry_path = current_entry.path();

        copy(&entry_path, &texture_mods_folder)?;

        mod_files.push(entry_path);
    }

    let installing_mod = Mod::new(
        answer,
        mod_files,
        true,
        ModType::Textures,
    );

    Ok(installing_mod)
}

pub fn install_player_model(dtt_dat_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>>  {
    let answer = ask_mod_name()?;

    let pl_mods_folder = game_path.join("/data/pl");

    let mut mod_files: Vec<PathBuf> = vec![];
    for entry in read_dir(dtt_dat_folder_path)? {
        let current_entry = entry?;
        let entry_path = current_entry.path();

        copy(&entry_path, &pl_mods_folder)?;

        mod_files.push(entry_path);
    }

    let installing_mod = Mod::new(
        answer,
        mod_files,
        true,
        ModType::PlayerModels,
    );

    Ok(installing_mod)
}

pub fn install_weapon_model(dtt_dat_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
    let answer = ask_mod_name()?;

    let wp_mods_folder = game_path.join("/data/wp");

    let mut mod_files: Vec<PathBuf> = vec![];
    for entry in read_dir(dtt_dat_folder_path)? {
        let current_entry = entry?;
        let entry_path = current_entry.path();

        copy(&entry_path, &wp_mods_folder)?;

        mod_files.push(entry_path);
    }

    let installing_mod = Mod::new(
        answer,
        mod_files,
        true,
        ModType::WeaponModels,
    );

    Ok(installing_mod)
}

pub fn install_world_model(dtt_dat_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
    let answer = ask_mod_name()?;

    let bg_mods_folder = game_path.join("/data/bg");

    let mut mod_files: Vec<PathBuf> = vec![];
    for entry in read_dir(dtt_dat_folder_path)? {
        let current_entry = entry?;
        let entry_path = current_entry.path();

        copy(&entry_path, &bg_mods_folder)?;

        mod_files.push(entry_path);
    }

    let installing_mod = Mod::new(
        answer,
        mod_files,
        true,
        ModType::WorldModels,
    );

    Ok(installing_mod)
}

pub fn install_cutscene_replacements(usm_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
    let answer = ask_mod_name()?;

    let cutscene_mods_folder = game_path.join("/data/movie");

    let mut mod_files: Vec<PathBuf> = vec![];
    for entry in read_dir(usm_folder_path)? {
        let current_entry = entry?;
        let entry_path = current_entry.path();

        copy(&entry_path, &cutscene_mods_folder)?;

        mod_files.push(entry_path);
    }

    let installing_mod = Mod::new(
        answer,
        mod_files,
        true,
        ModType::CutsceneReplacements,
    );

    Ok(installing_mod)
}

pub fn install_reshade_preset(preset_folder_path: PathBuf, game_path: &PathBuf) -> Result<Mod, Box<dyn std::error::Error>> {
	Ok(Mod::new(String::from("Texture"), vec![], true, ModType::Textures))
}



/* -------------------------- */
/*   INSTALLATION FUNCTIONS   */
/* -------------------------- */

fn ask_mod_name() -> Result<String, std::io::Error> {
	println!("Insert name of the mod that you are installing (choose anything you want, will be used as identifier)");
	print!("Name: ");
	stdout().flush()?;

	let mut answer = String::new();
	stdin().read_line(&mut answer)?;
	Ok(answer)
}

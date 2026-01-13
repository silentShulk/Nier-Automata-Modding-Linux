# HOW TO IMPLEMENT MOD HELPER

## FEATURES

### Mod Installation
- Receive the path to a folder, the folder will the be checked to se if there's a mod inside
- If there's a mod check which type of mod it is
- If it is a known mod ask the user for a name and add in a Mod List the name and files 
- After receiving the name follow the installation method for that type of mod
  - For models mods: put in the respective sub-data folder
  - For textures mods: put in the SK_Res/textures/inject folder
  - For reshade presets: NEED TO UNDERSTAND HOW TO INSTALL RESHADE

### SPECIAL MOD INSTALLATION
Should provide automatic installation for reshade, LodMod, Bande-Desineé and other mods that require dll injecting

### Mod Uninstallation
- Receive a mod name
- Check if that name is in the list
- If it is, get a list of all the files of that mod and remove them from the game's directory

### Print a list of mods
- Get a list of mods filtered by type form the list
- Print them in the terminal
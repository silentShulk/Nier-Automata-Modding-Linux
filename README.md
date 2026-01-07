# NieR: Automata Modding Linux

This is a tutorial and script utility for modding NieR: Automata on Linux.

| Distribution | Status     | Runner              |
| ------------ | ---------- | ------------------- |
| Arch         | ✅ Working | Proton Experimental |
| Fedora       | ✅ Working | Proton Experimental |

> ⚠️ The methods and scripts in this repository may still work for other distros.
> If you find that it works for your distribution then please update the `README.md`
> with this information or inform the maintainer.

## Methods

There are two ways to install the dependencies. You can either:

1. Run the auto-installer script for an easy installation.
2. Follow the tutorial below for manual installation.

The binaries and DLL in the project have been sourced from their official websites.
Nevertheless, I encourage you to check [Virus Total](https://www.virustotal.com) before trusting and binaries online.

| Binaries / DLL    | Version             | Source                                                                                              |
| ----------------- | ------------------- | --------------------------------------------------------------------------------------------------- |
| VC_redist.x64.exe | v14.0               | [Microsoft](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170) |
| VC_redist.x86.exe | v14.0               | [Microsoft](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170) |
| NieRAutomata.exe  | Patched EXE (Win10) | [Nexus Mods](https://www.nexusmods.com/nierautomata/mods/70)                                        |
| d3d11.dll         | v25.12.2.5          | [GitHub](https://github.com/SpecialKO/SpecialK/releases/tag/SK_25_12_2_5)                           |

You may also download the files from the sources above and follow the manual tutorial.

**Note:** `d3d11.dll` has been renamed from its original name. Formerly `SpecialK64.dll`,
needs to be renamed for the modding to work. Please refer to the official
`Special K` documentation to know why this is the case.


## Getting started

Start by cloning the repository locally:

git command:
```bash
git clone https://github.com/silentShulk/Nier-Automata-Modding-Linux.git
```
GithubCLI:
```bash
gh repo clone silentShulk/Nier-Automata-Modding-Linux
```

You will be met by the following folder structure:

```
/Nier-Automata-Modding-Linux
  | bin
    \_ NieRAutomata.exe
    \_ VC_redist.x64.exe
    \_ VC_redist.x86.exe
  | lib
    \_ d3d11.dll
  | LICENSE
  | README.md
  | install-prerequisites.sh
```

### Explanation of the files
- bin folder (binaries/executables)
	- **NieRAutomata.exe** -> Modded exe of the game, former WolfFileSizeLimitBreaker
	- **VC_redist.x64.exe and .x86.exe** -> Installers for the Microsoft C++ tools, 64 and 32 bits
- lib folder (libraries/dlls)
	- **d3d11.dll** -> This is the SpecialK dll, it was renamed from SpecialK64.dll
- **install-prerequisites.sh** -> This is the install script that automatically puts all the files in your NieRAutomata directory and launches your game

_Why are this files needed?_

`NieRAutomata.exe`:

The new exe is needed because PlatinumGames executables have a limit to how many dlls can be linked to it, and if you want to link mods' dlls to the game (which a fancy way for saying "installing mods") you need to have something that does this
It is possible to use SteamTinkerLaunch to do the same thing, but it is a different process for each distro and is difficult to make it work

`VC_redist.x64.exe` and `VC_redist.x86.exe`:

The installers for the C++ chain tools are needed because of how Automata is made, it relies on Microsoft stuff because it is a port exclusively for Windows, it isn't native on Linux

`d3d11.dll`:

SpecialK is the "Swiss Army Knife" of PC gaming, thanks to it we can load FAR (Fix Automata Resolution) mod which allows for texture injecting so that we can fix the horrible MGR:R-level textures present in the game
The name is a convention, dlls with that name get automatically loaded without needing to do anything strange

`install_prerequisites.sh`:

It is easier for you to install everything this way 😊
After launching the game SpecialK will create new folders that it needs, so don't worry if you see new folders like FAR, logs, ecc...


## Automated installation

This process is automated, but if something breaks it's harder to find what broke.

> Assuming Wine Windows Program Loader is installed in your system.

Make sure the script is executable, then execute it:

```bash
chmod +x install-prerequisites.sh && ./install-prerequisites.sh
```

Command explanation:
- `chmod +x <file-path>` -> This makes the file you give it executable
- `<command1> && <command2>` -> This is the syntax for executing a command only if the previous command was ran successfully
- `./<filename>` -> "./" indicates the current directory, by passing the name of script the file will be executed

The script will move the necessary files to your NieR: Automata game folder.
Wine will prompt you twice to install .NET dependencies,  you must click install.


## Manual installation

This is process is longer, but if anything doesn't work you will notice
immediately and will know exactly what is broken. Basic terminal knowledge is expected.

After cloning the repository, `cd` (Change Directory) into it:

```bash
cd Nier-Automata-Modding-Linux
```

You will need to make sure that the binaries are executable:
`chmod +x <file-path>` -> This makes the file you give it executable

```bash
chmod +x bin/VC_redist.x64.exe bin/VC_redist.x86.exe
```

Install the VC++ Redistributable for x64 (64-bit) architecture:

```bash
./bin/VC_redist.x64.exe
```

Install the VC++ Redistributable for x86 (32-bit) architecture:

```bash
./bin/VC_redist.x86.exe
```

Now you need to find out where NieR: Automata is installed. With Steam open,
`right-click the game -> Manage -> Browse local files`. 

You can also find it through the terminal, but, if you have steam games installed in a folder that isn't the default one steam chooses, you need to know where you installed it. Assuming it is somewhere in your home folder (the default path can be found like this):

```bash
find /home -type d -name NieRAutomata # ../steamapps/common/NieRAutomata
```

Then you must copy the SpecialK Dynamic Link Library (`.dll`) to the game path:

```bash
cp lib/d3d11.dll <PATH_TO_NIER_AUTOMATA_FOLDER_HERE>
```

Rename the original `NieRAutomata.exe` executable as a backup
The move (`mv`) command can rename a file by moving said file in the same folder but changing the name at the end of the path:

```bash
mv <PATH_TO_NIER_AUTOMATA_FOLDER_HERE>/NieRAutomata.exe <PATH_TO_NIER_AUTOMATA_FOLDER_HERE>/NieRAutomata(original).exe
```

Finally, move the new patched executable into the NieR: Automata directory:

```bash
cp bin/NieRAutomata.exe <PATH_TO_NIER_AUTOMATA_HERE>/NieRAutomata.exe
```

Launch the game


## Post-installation

1. After launching the game, it should start normally and in the loading screen you should see the SpecialK UI.

2. Press `Win + Shift + Backspace` to open SpecialK interface.
   I suggest going into the Framerate section and removing the 60fps cap, then in the Framerate Limiter section set the framerate cap at your monitor's refresh rate, so that NieR: Automata runs smoother without asking your GPU for frames that your monitor can't show.

3. Your NieR: Automata folder should now have this files (entries ending in / are folders)

```
../steamapps/common/NieRAutomata
  | data/
  | logs/
  | SK_Res/
  | Version/
  | FAR.ini
  | NieRAutomata.exe
  | NieRAutomata(original).exe
  | steam_api64.dll
```

You can now install any mod you want by following the tutorials on nexus mods.

> Some mods (those that require Windows specific stuff) are probably not made for Linux and
> will therefore not work. Neither this project nor the maintainer guarantees that all mods will work.
> 
> THE NIER AUTOMATA MOD HELPER (NAMH) ISN'T SUPPORTED ON LINUX AND THE AUTHOR DOESN'T PLAN ON MAKING A PORT. This means all mod must be installed manually


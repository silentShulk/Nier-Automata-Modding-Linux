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

```bash
git clone https://github.com/silentShulk/Nier-Automata-Modding-Linux.git
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

## Automated installation

This process is automated, but if something breaks it's harder to find what broke.

> Assuming Wine Windows Program Loader is installed in your system.

Make sure the script is executable, then execute it:

```bash
chmod +x install-prerequisites.sh && ./install-prerequisites.sh
```

This will move the necessary files to your NieR: Automata game folder.
Wine will prompt you twice to install .NET dependencies, which you must click install.

## Manual installation

This is process is longer, but if anything doesn't work you will notice
immediately and will know exactly what is broken. Basic terminal knowledge is expected.

After cloning the repository, `cd` into it:

```bash
cd Nier-Automata-Modding-Linux
```

You will need to make sure that the binaries are executable:

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
`right-click the game -> Manage -> Browse local files`. Or using the terminal:

```bash
find /home -type d -name NieRAutomata # ../Steam/steamapps/common/NieRAutomata
```

Then you must move the Dynamic Link Library (`.dll`) to the game path:

```bash
mv lib/d3d11.dll <PATH_TO_NIER_AUTOMATA_HERE>
```

Rename the original `NieRAutomata.exe` executable as a backup:

```bash
mv <PATH_TO_NIER_AUTOMATA_HERE>/NieRAutomata.exe <PATH_TO_NIER_AUTOMATA_HERE>/NieRAutomata(original).exe
```

Finally, move the new patched executable into the NieR: Automata directory:

```bash
mv bin/NieRAutomata.exe <PATH_TO_NIER_AUTOMATA_HERE>/NieRAutomata.exe
```

## Post-installation

1. Launch the game. You should now see the SpecialK UI
   If you can launch a save with nothing breaking then you can close the game.

2. Press `Win + Shift + Backspace` to open SpecialK interface.
   I suggest going into the Framerate Limiter section and setting the framerate cap at your monitor's refresh rate, so that NieR: Automata runs smoother without asking your GPU for frames that your monitor can't show.

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

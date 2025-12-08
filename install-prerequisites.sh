#!/bin/bash


# ---------------------------------- #
#   INSTALLING MICROSOFT C++ TOOLS   #
# ---------------------------------- #

# Install 64 bit
./Modding-Setup-Files/VC_redist.x64.exe

#Install 32 bit
./Modding-Setup-Files/VC_redist.x86.exe


# -------------------------------- #
#   CHANGING THE GAME EXECUTABLE   #
# -------------------------------- #

# Change the name of the default exe
mv $HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata.exe $HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata(original).exe

# Put the WolfLimitBreaker exe in the game directory
cp ./Modding-Setup-Files/NieRAutomata.exe $HOME/.local/share/Steam/steamapps/common/NieRAutomata/



# ------------- #
#   SPECIAL K   #
# ------------- #

# Put the dll in the games's directory
cp ./Modding-Setup-Files/d3d11.dll $HOME/.local/share/Steam/steamapps/common/NieRAutomata/



# --------------- #
#   LAUNCH GAME   #
# --------------- #

# Find NieRAutomata SteamID
set -gx nier_appid (find ~/Games/steam/steamapps/ -maxdepth 1 -type f -name '*.acf' -exec awk -F '"' '/"appid|name/{ printf $4 "|" } END { print "" }' {} \; | column -t -s '|' | sort -k 2 | grep -i NieRAutomata)

# Launch game
steam steam://rungameid/{$nier_appid}


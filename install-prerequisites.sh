#!/bin/bash

# ---------------------------------- #
#   INSTALLING MICROSOFT C++ TOOLS   #
# ---------------------------------- #

# Install 64 bit
./bin/VC_redist.x64.exe

#Install 32 bit
./bin/VC_redist.x86.exe

# -------------------------------- #
#   CHANGING THE GAME EXECUTABLE   #
# -------------------------------- #

# Change the name of the default exe
# This is assuming `.local/share/Steam` is even the default directory.
# Some distributions have other paths. It is also dependant on the source Snap, Flatpak, etc...
mv "$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata.exe" "$HOME/.local/share/Steam/steamapps/common/NieRAutomata/NieRAutomata(original).exe"

# Put the WolfLimitBreaker exe in the game directory
cp ./bin/NieRAutomata.exe "$HOME/.local/share/Steam/steamapps/common/NieRAutomata/"

# ------------- #
#   SPECIAL K   #
# ------------- #

# Put the dll in the games's directory
cp ./lib/d3d11.dll "$HOME/.local/share/Steam/steamapps/common/NieRAutomata/"

# --------------- #
#   LAUNCH GAME   #
# --------------- #

# Launch game
steam steam://rungameid/524220

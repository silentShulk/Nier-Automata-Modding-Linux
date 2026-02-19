#!/bin/fish

# ---------------------------------- #
#   INSTALLING MICROSOFT C++ TOOLS   #
# ---------------------------------- #

printf "\nInstalling Microsoft C++ Tools\n"
sleep 2

# Install 64 bit
./bin/VC_redist.x64.exe

#Install 32 bit
./bin/VC_redist.x86.exe



# ---------------------------- #
#   Preparing game directory   #
# ---------------------------- #

printf "\nGetting game directory"
sleep 2

# Setting the game path
if test -e argv[1]
  # Setting game path to first argument if it exists
  printf "Setting game directory to $argv[1]\n"
  game_dir="$argv[1]"
else
  # Setting game path to a default
  printf "No game directory provided, defaulting to $HOME/.local/share/Steam/steamapps/common/NieRAutomata\n"
  game_dir="$HOME/.local/share/Steam/steamapps/common/NieRAutomata"
fi

# Check if game installation exists at the set game path
if test -d $game_dir
  printf "Game directory found in $game_dir\n"
else
  # Exit if no game installation found
  printf "Game directory not found in $game_dir, please provide a valid directory\n"
  exit 1
fi



# -------------------------------- #
#   CHANGING THE GAME EXECUTABLE   #
# -------------------------------- #

printf "\nPutting modded exe in game directory and changing name of original exe"
sleep 2

# Change the name of the default exe
mv "$game_dir/NieRAutomata.exe" "$game_dir/NieRAutomata(original).exe"

# Put the WolfLimitBreaker exe in the game directory
cp ./bin/NieRAutomata.exe "$game_dir"



# ------------- #
#   SPECIAL K   #
# ------------- #

printf "\nPutting SpecialK dll in game directory"
sleep 2

# Put the dll in the games's directory
cp ./lib/d3d11.dll "$game_dir"



# --------------- #
#   LAUNCH GAME   #
# --------------- #

printf "\nLaunching game"
sleep 2

# Launch game
steam steam://rungameid/524220

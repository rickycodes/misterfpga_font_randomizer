#!/bin/bash

FAT=/media/fat/

# make a backup
cp "$FAT"MiSTer.ini "$FAT"MiSTer_original.ini
# strip file of comments since rust_ini can't handle them
sed -i '/^[ \t]*;/d' "$FAT"MiSTer_original.ini
# randomize and replace font
"$FAT"Scripts/misterfpga_font_randomizer "$FAT"MiSTer_original.ini
# copy now updated "_original" back over
cp "$FAT"MiSTer_original.ini "$FAT"MiSTer.ini
# remove backup
rm "$FAT"MiSTer_original.ini

reboot

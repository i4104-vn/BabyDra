#!/bin/bash

mkdir -p ~/.config/labwc ~/.local/share/themes
cp -r configs/themes/BabyDra ~/.local/share/themes/ 
mkdir -p ~/.config/labwc/themes
cp -r configs/labwc/themes/* ~/.config/labwc/themes/
cp configs/labwc/{rc.xml,themerc-override,switcher.sh,autostart} ~/.config/labwc/ 
chmod +x ~/.config/labwc/switcher.sh ~/.config/labwc/autostart 
killall switcher.sh || true
~/.config/labwc/switcher.sh &
labwc --reconfigure

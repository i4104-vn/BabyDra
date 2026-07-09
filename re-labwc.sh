#!/bin/bash

mkdir -p ~/.config/labwc ~/.local/share/themes
cp -r configs/themes/BabyDra ~/.local/share/themes/ 
cp configs/labwc/{rc.xml,themerc-override,theme-monitor.sh} ~/.config/labwc/ 
chmod +x ~/.config/labwc/theme-monitor.sh 
labwc --reconfigure

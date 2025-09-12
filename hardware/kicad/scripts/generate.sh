#!/bin/bash

# paru -S kibot kicad-interactive-html-bom-plugin librsvg
# paru -R kibot
# pip install --break-system-packages --no-compile kibot

~/.local/bin/kibot -c hardware/kicad/scripts/kibot.yaml -e hardware/kicad/Yatchy/Yatchy.kicad_sch -b hardware/kicad/Yatchy/Yatchy.kicad_pcb
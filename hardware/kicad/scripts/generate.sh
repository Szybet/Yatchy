#!/bin/bash

# paru -S kibot kicad-interactive-html-bom-plugin librsvg
# paru -R kibot
# pip install --break-system-packages --no-compile kibot

~/.local/bin/kibot -c hardware/kicad/scripts/kibot.yaml -e hardware/kicad/Yatchy/Yatchy.kicad_sch -b hardware/kicad/Yatchy/Yatchy.kicad_pcb

~/.local/bin/kibot -c hardware/kicad/scripts/kibot.yaml -e hardware/kicad/modules/default/module.kicad_sch -b hardware/kicad/modules/default/module.kicad_pcb
mv hardware/read-only/module-ibom.html hardware/read-only/module-default-ibom.html
mv hardware/read-only/module-schematic.pdf hardware/read-only/module-default-schematic.pdf
mv hardware/read-only/module-schematic.svg hardware/read-only/module-default-schematic.svg

~/.local/bin/kibot -c hardware/kicad/scripts/kibot.yaml -e hardware/kicad/modules/BTBMHM/module.kicad_sch -b hardware/kicad/modules/BTBMHM/module.kicad_pcb
mv hardware/read-only/module-ibom.html hardware/read-only/module-BTBMHM-ibom.html
mv hardware/read-only/module-schematic.pdf hardware/read-only/module-BTBMHM-schematic.pdf
mv hardware/read-only/module-schematic.svg hardware/read-only/module-BTBMHM-schematic.svg

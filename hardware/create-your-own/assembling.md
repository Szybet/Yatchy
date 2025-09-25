## **A reminder, all of this is provided without any warranty. If something goes wrong and the PCB is bad, wrong components, not my fault**

# Assembling Yatchy
There are 2 ways to solder a Yatchy (in a reasonable way):
- Solderimg manually:
  - Allows for bigger control over the process, allows for more debugging capabilities, more testing options (I will explain it below)
  - Takes more time
  - Cheaper
- Via a stencil
  - Less debugging options, less testing options, less control of what's going on (which doesn't mean it's not possible, simply compared to manually)
  - Faster
  - More expensive

Some more notes:
- As for the skill level required, yh, idk, both are hard, here you solder everything manually but in stencil you need to repair things that didn't go well, so idk, decide for yourself
- In this guide I will focus on manual, but also say from time to time things specific to stencil. If you use stencil for soldering many yatchies, you should still solder at least one manually to understand what is going on.
- Order of soldering things is pretty much up to you and your choosing, sometimes I will note things that I suggest. Just do it so it's the easiest for you
- If a test is too hard for you, it could be skipped, but if something later doesn't work, it will be a lot harder to debug what is actually going on, it could even result in damaging more components
- I will not talk things like "If you power the device via 5V USB, you can't have a power supply connected on the battery terminals, otherwise the charger will start charging and things can go messy". Common sense, common electronical/arduino/esp32 level knowledge required

Things like:
- Equipment I use, or is needed
- What soldering skills are needed
- Where to view the PCB and schematic (while soldering, you should probably open it to know which components to grab)

Are discussed in other files. I will not link them, as you should read everything anyway, so there are no suprises later.

# Yatchy "sections"
Yatchy (as any other electronic device really) consists of sections. Some depend on each other, the point is you pretty much can't (and shouldn't) (until you have the skill and knowledge to know what you are doing) debug things when soldering half of them. Also don't power the device when half of it is soldered. Things may simply not work or just damage itself (rarely)

The schematic is also splitted into those sections, but sometimes I modify the sections below for reasons

### Power
<img width="701" height="741" alt="image" src="https://github.com/user-attachments/assets/9b6bb354-734e-49c3-8ce5-541b4d5a308b" />

Converts either 4.2V from the battery or 5V from USB to 3.3V used by the rest of the board. You don't need other sections (Battery, USB) to provide these voltages to it, you can simply do it via a wire (where the voltages would normally be provided from). All other sections depend on this section, but if you have an external, good and stable 3.3V source, you can power it directly to the 3.3V line. I usually solder the 3.3V and GND wire here:

<img width="212" height="301" alt="image" src="https://github.com/user-attachments/assets/6e9391d2-11fa-4a8b-8b43-8b4eb6812975" />

so if you do the wire, you can skip this section for now

### esp32c6
<img width="727" height="749" alt="image" src="https://github.com/user-attachments/assets/ce3f1ae5-505c-482d-b350-5c57480179b0" />

Pretty much the hardest section, needs to be soldered via hot plate (hot air is too hard for me). Apply solder on both sides, on the chip itself only a thin layer, place it ideally and give it a go...

**Testing**
- First, if you can, measure the power consumption. It should be around 20-30mA (just this section, more sections more power). If it's like above 50mA, you are in danger, probably a short somewhere, probably already smoked
- Second, check if there is communication:
  - if you have a UART-USB converter (if not, you can reuse an arduino, esp32 devboard, RPI for that, but I won't explain it here, google it), solder GND, RX, TX (below the flex cable), with baudrate 115200 check if there is any communication. You should see some ROM messages
  - if you soldered the USB section, if you connect the device, it should connect itself and disconnect all the time. That's normal, there is no program so it resets itself. When opening the serial device that appeared on the PC, baudrate 115200 you should also see ROM messages over and over again.

If you did not solder anything else (well you need power in any form, the USB section is allowed and optional, as told above) (so stencil users, not for you), lucky you. There is a test-program which allows for detecting shorts & checking connections with a multimeter. It's located in the test-program directory in this directory. Grab it to your PC

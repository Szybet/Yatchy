## **A reminder, all of this is provided without any warranty. If something goes wrong and the PCB is bad, wrong components, not my fault**

# Assembling Yatchy
There are 2 ways to solder a Yatchy (in a reasonable way):
- Soldering manually:
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
- The sections are from top to bottom of order of what needs to be soldered
- Order of soldering things (in a section) is pretty much up to you and your choosing, sometimes I will note things that I suggest. Just do it so it's the easiest for you
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

Pretty much the hardest section, needs to be soldered via hot plate (hot air is too hard for me). Apply solder on both sides, on the chip itself only a thin layer, place it ideally and give it a go... You can skip the main big ground pad in the middle, not really needed

**Testing**
- First, if you can, measure the power consumption. It should be around 20-30mA (just this section, more sections more power). If it's like above 50mA, you are in danger, probably a short somewhere, probably already smoked
- Second, check if there is communication:
  - if you have a UART-USB converter (if not, you can reuse an arduino, esp32 devboard, RPI for that, but I won't explain it here, google it), solder GND, RX, TX (below the flex cable), with baudrate 115200 check if there is any communication. You should see some ROM messages
  - if you soldered the USB section, if you connect the device, it should connect itself and disconnect all the time. That's normal, there is no program so it resets itself. When opening the serial device that appeared on the PC, baudrate 115200 you should also see ROM messages over and over again.
  - with UART or USB, after typing `esptool --after no_reset chip_id` (the esptool cli command) (you might need to type it many types to catch, as the esp32c6 resets) (This command resets the device into bootloader mode, this is the state that it needs to be flashed in). If it can't catch it, you might need to reset the device into bootloader mode manually with those pins:
 
<img width="51" height="160" alt="image" src="https://github.com/user-attachments/assets/19539714-0b2e-4646-b231-9fc3ceb61712" />

Bootloader and reset pin. If you can't google it out, let me know, I will write or even record a proper tutorial

If you did not solder anything else (well you need power in any form, the USB section is allowed and optional, as told above) (so stencil users, not for you), lucky you. There is a test-program which allows for detecting shorts & checking connections with a multimeter. It's located in the test-program directory in this directory. Grab it to your PC, then:
- All of this tested only on linux. Windows could work, idk
- Install the rust programming language. Your cli should be able to execute the cargo command
- Follow the README.md in the test-program directory
- in Cargo.toml, in features section choose what you have (do you have usb, or uart (don't both), or you have i2c section. Add them to default, follow the toml syntax (or ask google/AI)
- in .cargo/config.toml comment and uncomment the `runner` line for your communication method
- if everything is correct, run in test-program directory `cargo run --release` - it should succeed and flash the program (You might need to reset it to bootloader mode), you should see logs from the program booting. Now you have a few commands to choose from (no backspace support lol):
  - `self_check_gpio` - It sets one gpio to high, the rest as inputs and checks if the inputs are high. If it says it's high, it's worth checking if there is a short (After disconnecting the device from power...). If there is not a short, cool, then it's just floating magic. Was worth to check anyway.
  - `gpioX` - Where X is number of the gpio. For available gpios for your setup, look up `src/flex_io.rs` the `pub struct FlexIo<'a>` line. If a gpio is behind a feature which you have not enabled, then it's not available to check. If you type type a gpio command, it will trigger it high and low every second. Check with a multimeter if the voltage jumps, if not, there is no connection you need to resolder the whole chip. (you can use soldering iron to maybe fix one pin, but not more, even with special footprint it's hard, but possible). To know where to connect the multimeter, open kicad and trace what goes where using the schematic and pcb view, the "`" button can be handy too.
  - `exit` - exits the current action. You need to type it after every action to switch to another

Now (+ USB section) (even with stencil) you should be able to flash and compile inkwatchy, which will die inside because nothing else is present, so we need to enable testing mode:
- in src/defines/condition.h set LP_CORE to 0 for now, later, to get the lp core, join the discord server, that where it's disributed
- follow normal inkwatchy setup, but don't flash things, just build everything
- in src/defines/config.h set DEBUG to 1, WAIT_FOR_INPUT to 1 and SIMPLE_DEEP_SLEEP_TEST to 1
- in src/other/debug/debugMain.cpp in the section of `#if SIMPLE_DEEP_SLEEP_TEST && (WAIT_FOR_INPUT || WAIT_FOR_MONITOR)` turn off everything (so to 0) (MCP, ACC, etc)
- Flash
- Open monitor
- type 123
- it should go to sleep, it won't appear in in USB until a power cycle

This will allow us to test things later

### USB
<img width="720" height="758" alt="image" src="https://github.com/user-attachments/assets/3c33a9d1-bd80-44e2-9392-575c3a737ffb" />

the USB pins sometimes don't want to catch solder, use more flux and higher temperature

To test it, it requires a powered, from any source esp32c6 section. Testing it is described in that section too. If USB connection doesn't work, on linux you can look up the dmesg command for more info. Probably the connection of D+ D- pins are bad, or the TVS diodes are bad. If 5V doesn't appear from a usbc-usbc charger cable, then something is wrong with the resistors.

### I2C
<img width="743" height="811" alt="image" src="https://github.com/user-attachments/assets/212459c3-ab04-4524-b2a4-e523dfa4aeb6" />

simply 2 resistors. Can't be tested, if gpio expander or Acc doesn't work, maybe those are bad, as they depend on this

### Gpio expander (mcp23018)
<img width="720" height="758" alt="image" src="https://github.com/user-attachments/assets/fd2d6e5e-6504-4a11-a64b-85c4492cbca8" />

can be soldered manually or with hotplate. Make sure with microscope or magnifying glass that the pins are soldered properly

Requires i2c and esp32c6 (so also, uhm, power) sections to be tested. To test, in debugMain.cpp set IS_MCP and IS_MCP_CONFIGURATION to 1, reflash, you should see some logs about registers or failures if something is wrong. This test doesn't check connections, just if there is communication. That's why you need to make sure the pins are connected anyway!

### Screen
<img width="788" height="830" alt="image" src="https://github.com/user-attachments/assets/343a09b3-5220-4ef0-808e-04ac3807d8f6" />

Here I propose soldering the flex connector first, as its the hardest. Check for shorts, check if the pins are soldered (if they can be moved a bit with idk something sharp, then its not soldered). If there is solder behind the pins, use a solder wick to catch it. Clean flux also inside the connector, open it's "brackets"

the rest is easy, the coil at the end.

the flex cable connector needs cutting off a bit of plastic here, with a knife, scalpel:

<img width="478" height="302" alt="image" src="https://github.com/user-attachments/assets/41970758-6fa1-430d-8945-2bfe1de5223e" />

to test, requires: esp32c6, mcp23018 (so power, i2c, usb too).

You can test the screen by inserting it "straight", not on the back. Look up pictures in the repo on how far it should be inserted. Be carefull while inserting it, apply the force equally when closing the brackets. For detaching use thin tweezers, toothpick and do it slowly, as too much force can break the connector

to test, in debugMain.cpp enable IS_SCREEN (+ mcp things are also needed) after flashing and everything, a black-white circle should appear on the screen. Grey screen indicates problems. even one loose pin can break everything. The mcp23018 also needs to work for the screen to work

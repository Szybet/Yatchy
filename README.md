<h1 align="center">
  Yatchy
</h1>
<p align="center">
  <b>Y</b>our W<b>atchy</b>, fixed and improved, even made modular
</p>

<p float="left" align="middle">
  <img src="/img/Yatchy1.png" width="340,5" height="384"/>
</p>

Yatchy is a full redesign, from the ground up of the well known [watchy](https://watchy.sqfmi.com/). It focuses on fixing known issues and expanding the wasted potential of it.

#### Key points:
- USBC instead of micro usb
- Better buttons, they won't break easily
- [Better battery life](https://github.com/Szybet/Yatchy/blob/main/hardware/power-consumption/README.md)
- Hardware hackable
- Fixed many issues and improved minor things

<details>
  <summary><h3>Pictures of my prototype</h3></summary>



- <sub>The battery is on the side because it's easier for me to measure power consumption</sub>
- <sub>The glue is for the vibration motor which is detached in the picture</sub>

<p align="center">
  <img src="https://github.com/user-attachments/assets/3ae1a087-5366-4c8e-a037-5b8754a2d5d1" width="45%" style="display:inline-block; margin-right: 5px;">
  <img src="https://github.com/user-attachments/assets/8258f4e3-1292-4f11-8b0e-b0b4048dbdf4" width="45%" style="display:inline-block;">
</p>

</details>

Yatchy features, compared to 2.0 watchy:

<sub>watchy v3 <a href="https://github.com/Szybet/WatchySourcingHub#original-watchy-v3">is a joke</a>, It's design files are not released, only the schematic, also it has many flaws so I don't even compare to it</sub>

<details>
  <summary><h2>For regular users</h2></summary>

- Heavy increase in battery life, even with the screen updating every minute <sub>(The esp32c6 has a riscv lp core, which i connected to the screen, which means the device never really wakes up fully to only update the screen, not only that i fixed sqfmi flaws (Using 2 voltage dividers for example) I also use the external crystal clock for the RTC which means no RTC IC... and many more such things)</sub>
- Better buttons, more solid, shouldn't and probably won't break that much if at all. They are also very quiet. If you like the tactile feel of the original buttons, you won't like these ones. Luckly vibration motor feedback is possible.
- Solid USB-C port <sub>(It's THT soldered)</sub>
- no more usb compability problems <sub>(Because the esp32c6 uses native jtag programming)</sub>
- better detection of charging, charge finishing. <sub>(In software it was hacky on the original watchy, there were problems with it, never worked good)</sub>
- support for more wireless protocols, home automation ones, wifi 6 too
- ~~Mouse bites to more easily make your yatchy smaller, of the size of the screen and not the watchy form factor~~ <sub>JLCPCB decided to treat it as a seperate design and charge double, so I removed it. Stupid.</sub>
- TVS diodes, voltage spike protection - which means no more destroyed devices because watchy doesn't follow any USB spec and allows your device to burn down...
- More precise time, based on some loose math and experiments, it should drift only a minute after a month, compared to the watchy its a lot better

</details>

<details>
  <summary><h2>For advanced users</h2></summary>

- **A module area**, with almost all exposed pins from the esp, power lines, and many gpio pins thanks to the expander IC - You can create your own module, increase the capability of you yatchy without modifying everything inside - solar panel module, encoder instead of the button, sd card module, frontlight module, torchlight module, speaker module, microphone module, some environmental sensors. The only limitation is your imagination (And the size of the module)
   - The default module (In the picture above) allows for for attaching small things without the need for a custom PCB
- All the components are newer, still available and produced. They are also listed, with links to mauser / tme. Ordering a Yatchy PCB and the parts, soldering them themself is possible and easier than the watchy
- All QFN packages IC's on the board have increased pad sizes, so if you are skilled enough to solder QFN packages, here it will be easier
- I used traces teardrops, so more solid traces & pads
- A [hardware test program](https://github.com/Szybet/Yatchy/tree/main/test-program), which allows testing if all the pads of the esp32c6, which is the hardest part to solder are soldered correctly
- JTAG debugging via usb, yay

</details>

<details>
  <summary><h2>Flaws</h2></summary>

- No battery connector, the module pads are small - so it's not for everyone, harder to use / assembly (solder!) for beginners
- There is no dedicated place for the motor, it's just Yolo on the components with some glue. Soldering the motor cables also is not easy
- It's watchy like but not watchy case compatible, the size and form factor is the same but the obvious obstacle is USBC, even with it the buttons are different and placed a little off to save some space
- It's a 4 layer PCB, so a bit more costly (With JLCPCB it's still cheap)
- No easy hard reset option, you will need to short pins, but if you need to do it, then you made something really wrong. <sub>When developing the Yatchy, I only once needed a full reset</sub>
- A complicated license

</details>

## Software
It's fully supported by [InkWatchy](https://github.com/Szybet/InkWatchy). Maybe a rust firmware in the future? We will see...

## Hardware sources, buying, reselling, license, battery life, time accuracy
Everything explained [here](https://github.com/Szybet/Yatchy/tree/main/hardware)

## Cases, 3D files of the pcb
[Cases here](https://github.com/Szybet/Yatchy/tree/main/3D/cases) and [pcb 3D model here](https://github.com/Szybet/Yatchy/tree/main/3D/pcb)

# Community
For any questions, feel free to ask in github issues or in this discord server:

<a href="https://discord.gg/6PUmRXZRGD">*atchy community on discord</a>

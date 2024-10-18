<h1 align="center">
  Yatchy
</h1>
<p align="center">
  Your Watchy, with all the flaws of the original fixed
</p>

<p float="left" align="middle">
  <img src="/img/Yatchy.png" width="340,5" height="384"/>
</p>

~~The device doesn't exist yet, No software was written but (Not the case anymore)~~ hardware was created for it with these things in mind, I doubt something will change - That's the reason I also avoid giving any "software" features here, also some claims are theoretic, no battery life has been measured for example.

Yatchy features, compared to 2.0 watchy:

<details>
  <summary><h2>For regular users</h2></summary>

- Heavy increase in battery life, even with the screen updating every minute (The esp32c6 has a riscv lp core, which i connected to the screen, which means the device never really wakes up fully to only update the screen, not only that i fixed sqfmi flaws (Using 2 voltage dividers for example) I also use the external crystal clock for the RTC which means no RTC IC... and many more such things)
- Better buttons, more solid and better clicking experience (The device doesn't exist yet, I will test the buttons, if they will be not as good as I want I will replace them)
- Solid USB-C port (It's THT soldered)
- no more usb compability problems (Because i use native esp32c6 jtag programming)
- better screen connector, no more "reattach your screen" problems
- module (more on it below) but on the default there is a module, which contains flash storage (possible easy updates ever the air, or storage for other things, like books) easy exposed i2c pins (large tht pins) and a RGB diode ;)
- better detection of charging (In software it was hacky on the original watchy, there were problems with it)
- axcelerometer which will allow for more precise gestures, while using the watchy* (more on it later...)
- support for more wireless protocols
- Mouse bites to more easily make your yatchy smaller, of the size of the screen and not the watchy form factor
- TVS diodes, voltage spike protection - which means no more destroyed devices because watchy doesn't follow any usb spec and allows your device to burn down...
- Following all the specs of the various IC's - on the watchy it was just Yolo no capacitors here

</details>

<details>
  <summary><h2>For advanced users</h2></summary>

- A module area, with almost all exposed pins from the esp, power lines, and many gpio pins thanks to the expander IC - The default module has been described above, but you can create your own, increase the capability of you yatchy without modifying everything inside - solar panel module, encoder instead of the button, sd card module, frontlight module, torchlight module, speaker module, microphone module, some environmental sensors. The only limitation is your imagination (And the size of the module)
- the low power core while using the high power core could be used to write portable apps for the yatchy (but we could achieve that with lua anyway I think)
- All the components are newer, still available and produced. Ordering a Yatchy PCB and the parts, soldering them themself is possible and easier than the watchy
- All QFN packages IC's on the board have increased pad sizes, so if you are skilled enough to solder QFN packages, here it will be easier
- I used via teardrops, so more solid traces & pads
- JTAG debugging via usb, yay

</details>

<details>
  <summary><h2>Flaws</h2></summary>

- No battery connector, the module pads are small - so it's not for everyone, harder to use / assembly for beginers
- It's watchy like but not watchy compatible, the size and form factor is the same but the obvious obstacle is USBC, even with it the buttons are different and placed a little off - Solution to it would be probably some glue, cutting off the sqfmi case a bit and it would work, maybe maybe... Whatever, you can order on jlcpcb a nylon case (which is already designed) for a stupid low price, only shipping cost is high.
- The accelerometer I picked was the only one available to me easily. It is different from the previous accelerometer, some features will be missing but some will be new. If that will be a deal breaker for you there are 2 options: the footprints of those axc are almost identical, maybe it's a easy swap? Also you could easily put the old acc on the module area yourself
- It's a 4 layer PCB, so a bit more costly

</details>

# Community

<a href="https://discord.gg/6PUmRXZRGD">Join the *atchy community on discord</a>

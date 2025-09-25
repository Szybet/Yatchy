## **A reminder, all of this is provided without any warranty. If something goes wrong and the PCB is bad, wrong components, not my fault**

# Ordering components & Price
There is a spreadsheet for it: https://github.com/Szybet/Yatchy/blob/main/hardware/components-calc.ods

**I already miscalculated the components in this spreadsheet a few times, so here is another disclaimer for you:**
![image](https://github.com/user-attachments/assets/6a74510d-f3cd-4ae4-a6da-d6a2e2a5fdbd)
While it's scary, simply checking with the latest Kicad files is a good idea anyway.

You need to adjust all values for yourself, after that it will give you a approximation of how much will it cost. Why it's not precise?
- Sometimes you need to order 1 component where the minimum is 5, so you order 5 (That's the case with pcb's)
- Sometimes it's advised to buy spare components if you screw something up (I Usually buy spares for all of them)
- Sometimes when ordering minimum + spares you hit the next price point, then it drops
- Sometimes, prices change
- Random taxes, discounts, shipping magic etc.
- Ordering things for more than 1 yatchy will make things cheaper, again, price tresholds, again, economy of scale (Small scale but scale). Also shipping will be splitteb between yatchies

So it's save to say it's precise up to +-15$, maybe even 20$

If you feel extra insecure and you don't trust me **(You shouldn't)** check the kicad files if the document misses a component

For me, the cost is arround 70-80$, without the labour and things like solder / flux

Also to note, mauser is not required, I was able to buy everything with TME + falstad (Without the accelerometer, lol). Also digikey looks like it has all the components

#### Note about accelerometers
the PCB supports both BMA530 and BMA456 (even BMA423... but this one is impossible to get). Generally, only BMA456 is suggested, the BMA530 is there as a backup if BMA456 is not available. Sadly, BMA530 is harder to solder, it's basically BGA (but BMA530 is 2 times more precise than BMA456, while BMA456 is 2 times more precise than BMA423 - the precission doesn't really matter for normal usage / step tracking)

### Choosing alternative components
If you want to choose alternative components (we are talking about capacitors / resistors, for transistors etc. you are on your own) follow this:
1. Follow the excel file, not the BOM file.
2. Generally, 5% tolerance is good, but there are exceptions:
- For the quartz capacitors, get the max possible tolerance (1%?)
- For screen circuit things, get better tolerances if it doesn't cost you much, because if something will not work, it will be hell to debug

### Motor
There is no dedicated place for the vibration motor. So it's simply yeet it on top of components (Isolation in between!). Also a small one is advised. I bought these ones: https://aliexpress.com/item/1005004948677826.html smallest ones I could find, they work, they do their job

# Ordering the PCB
### Warning
The PCB you are ordering might not be real life tested as I apply patches regularly but I don't order them, so as everything, all of this is provided without any warranty 

1. Install kicad version (specified in hardware/README.md) or later
2. Install this plugin https://github.com/bouni/kicad-jlcpcb-tools
3. Clone the repo / open the pcb project so you will see something like this (PCB on the pic is outdated)
- Of you don't understand the above, here is a step by step:
   - Get all the files from this repo, for example like this (Download zip)

![image](https://github.com/user-attachments/assets/5bdac971-abd1-4370-ba58-c796bbdb6b8d)

   - Extract it
   - Open in kicad the file "Yatchy.kicad_pro"
   - Then in kicad open the pcb file

![image](https://github.com/user-attachments/assets/7a05e957-d8bc-4559-ab97-ff867b3ab0d9)

4. Click this:

![image](https://github.com/user-attachments/assets/2e523c5f-6c82-4cd1-8e32-61e2babe8718)

5. Click this

![image](https://github.com/user-attachments/assets/b5acc341-297b-4d69-944a-330b62cc5b5f)

6. Now in the logs you will see where the magic zip file is

![image](https://github.com/user-attachments/assets/774431bb-d07c-4ec0-a7ff-8e8abeb8abb4)

7. Log into JLCPCB with an account
8. Go to https://jlcpcb.com/ and add the zip file

![image](https://github.com/user-attachments/assets/a6013d2f-5d88-40d1-97f8-891410bbb9ff)

9. You should see something like this

![image](https://github.com/user-attachments/assets/2b82a1f1-587e-45ee-a7ef-7403bf918025)

10. Now make sure those settings are matched:
- Layers: 4
- PCB Thickness: 1.0mm
- Mark on PCB: Remove mark (If it charges you more money for this option, then just switch back to Order number)

The rest defaults should be fine, but you can explore and check if everything is fine. A huge red flag is when it costs a lot (>12$ let's say), it shouldn't.

Now repeat the process with the module you want to attach, then order them rogether (The default module is here: https://github.com/Szybet/Yatchy/tree/main/hardware/kicad/modules/default)

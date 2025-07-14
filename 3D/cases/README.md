# Yatchy cases
If you create your own model, fork one, or anything else, please (in addition to putting it in your repository, on a 3D model website, if you feel like it, etc.) contribute it here so it won't get lost in the ether of poor search engines. Sharing files that can be modified easily (STEPs, not STLs) is also suggested, but not required.

### The files are in the folders in this directory

<h1>$${\color{red}Patches}$$</h1>
Because of the tight tolerances and different printers, filaments, some cases might have alternative parts that might fix your issues, read their description and decide for yourself, generally, I would advice using them

## Szybet's side case
<p align="center">
  <img src="Szybet-side/img/front.jpg" width="300px" style="display:inline-block; margin-right: 5px;">
  <img src="Szybet-side/img/back.jpg" width="300px" style="display:inline-block;">
  <img src="Szybet-side/img/case.gif" width="300px" style="display:inline-block;">
</p>

<details>
<summary>Fit and dissasembly demo</summary>
  
<p align="center">

https://github.com/user-attachments/assets/d5af4c48-378c-4553-9d58-3c49acbb4d74

</p>

</details>

Notes:
- Yes, it's designed for the battery to be on the side (and more space for module hackery), yes you can modify it for your needs
- The top snaps into the bottom case, no screws, it's tight and easy to dismantle
- The strap & battery case is printed from a flexible material, TPU in this example.
- Requires a 0.2 nozzle, a **very good 3D printer**. The tolerances are very tight. - for the case, the layer adhesion must be good otherwise it could break when buttons are inserted. (Semi transparent PETG has good layer adhesion, I think)
- The motor cover is for this motor: https://pl.aliexpress.com/item/1005004948677826.html?gatewayAdapt=glo2pol
- The module led cover is for the led to redirect the light to the side. I used a silver marker for better effect
- The side battery case has many files in it, they all should be printed as they are
- Needs strong layer adhesion, otherwise it will break near the edges. Some glue on the inside or transparent PETG does the job
- Patches nr 1 apply

License:

[Creative Commons (4.0 International License) Attribution—Noncommercial—Share Alike](https://creativecommons.org/licenses/by-nc-sa/4.0/)

### Szybet's brick case
<div align="center" style="white-space: nowrap; overflow-x: auto;">
  <img src="Szybet-brick/img/front.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
  <img src="Szybet-brick/img/side.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
  <img src="Szybet-brick/img/top.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
  <img src="Szybet-brick/img/render.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
</div>

Notes:
- This is a fork of "Szybet's side case", most of the notes from there apply here (license too), the design of the buttons, top is the same
- Yes, it's a simple brick, based on the side case, it's a base for modifying it for other cases with the battery inside
- The texture on the corners is for making them stronger, less breakable
- The strap still has cutouts for attaching something to them
- If the battery is inserted in a wrong way and the cables will touch the buttons, they might get stuck
- Patches nr 1 apply

### KitKat's rotated case
<div align="center" style="white-space: nowrap; overflow-x: auto;">
  <img src="Kitkat-rotated/img/front.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
  <img src="Kitkat-rotated/img/back.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
  <img src="Kitkat-rotated/img/side.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
  <img src="Kitkat-rotated/img/usb.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
</div>
<div align="center" style="white-space: nowrap; overflow-x: auto;">
  <img src="Kitkat-rotated/img/render-1.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
  <img src="Kitkat-rotated/img/render-2.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
  <img src="Kitkat-rotated/img/render-3.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
  <img src="Kitkat-rotated/img/render-4.jpg" style="width: 23%; min-width: 200px; display: inline-block; margin: 0 1%">
</div>

**The Images above are for the 150Mah case version**
<div align="center" style="white-space: nowrap; overflow-x: auto;">
  <img src="Kitkat-rotated/img/250mah.png" style="width: 50%; min-width: 200px; display: inline-block; margin: 0 1%">
</div>

**The Image above is for thh 250 mah case version**

Notes:
- The 250 mah battery is 30 x 20 x 5 mm and the 150 mah battery is 25 x 20 x 4 mm
- The strap holders on the 250 mah version are assymetric
- This is a fork of "Szybet's side case", most of the notes from there apply here (license too), the design of the buttons, top is the same
- Requires changing the config in Inkwatchy so the screen is rotated properly
- The pieces that hold the straps need to be glued onto the main body using superglue
- The strap still has cutouts for attaching something to them
- The battery fits inside but requires short wires for it to fit properly 
- The motor is attached to the inside of the case which can make it more difficult to dissasemble
- Larger USB-C plugs may not fit due to strap clearance
- case_front_v2 and better_strap_dot from patches 1 applies
- Those are pictures of a prototype
- **The buttons are objectively easier to access**
- The LED now shines outside of your wrist, not on it

### Case comparison
- without top case or the pieces that hold the straps

| Case         | Height (mm)  |
|------------------|--------------|
| Kitkat's Rotated 250 mah version | 10.29477     |
| Kitkat's Rotated 150 mah version | 8.70611      |
| Szybet's Side    | 9.55611      |
| Szybet's Brick   | 10.96346     |

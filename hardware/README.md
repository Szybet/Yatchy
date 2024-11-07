<sub>Read all of it!</sub>
## License
This work is licensed under a [Creative Commons (4.0 International License) Attribution—Noncommercial—Share Alike](https://creativecommons.org/licenses/by-nc-sa/4.0/)

Don't scream "IT'S NOT OPEN SOURCE", let me explain myself.

- First, this work is not a watchy fork, it's a complete redesign obviously inspired by the watchy form factor, but nothing more.
- I don't want SQFMI to steal this design and sell it. Not even If I could profit of it. His bad software, bad communication, sometimes bad and questionable hardware decisions have killed so much potential of the watchy.
- I don't want someone to just make money off it and give nothing back to the project. With that said, calm down, more on that in the commercial section below.
- I don't want for someone to rename it and take the credit for the hard work.

Is it too much to ask for? If there is a better license that fits my needs, let me know.

Oh did I mention **it's is open source anyway?** I share the full source, the schematic, the PCB source files, the needed* (more on it below) software. It's just not the pure open source definition, but the literal interpretation of it. Element of open source like contributing, forking etc. are still allowed and welcome, but with the restrictions above.

## The design of the Yatchy
Yatchy was designed to:
- Be created by yourself. That means it's adjusted to be hand solderable. Sooo that means changed footprints for easier hand soldering. If you can solder SMD without a problem and QFN packets aren't scary for you, but BGA is a bit, then it's for you
- Extremely hackable and modular. The main point is the module and all exposed pads for it

Those design choices have their own flaws:
- It's not possible to order a assembled yatchy from a manufacturer. Not only because of the module, but the modified footprints have not adjusted solder mask (And I don't have the experience and money to test it). With that in mind, let's go to the commercial section

## Commercial
As mentioned above, It's not possible to manufacture watchy on a mass scale. I don't even want to, I'm a student, I won't just start a company. I plan to sell a few Yatchy to community members and that's it.

But if you plan to create a Yatchy for yourself (because you have the skills required) you should consider creating some for people who can't do that. I will fully support it! Just remember to contribute some docs, code, help, donation and it will be fine :D

So "Commercial use" for the yatchy is using it as a cash grab, without giving anything back, which is not allowed.

If you plan to really mass produce them, then we should talk I ques.

## The ability to enforce the license
First some explanation:

Yatchy uses esp32c6 which has a lpcore which allows for updating time on the screen in a more efficient way, in a seperate program.

So all this talking is cool but it's just on paper, I can't enforce it, let's be real. That's why I will lock the full source code of the Yatchy lp program until I make back of what I invested. It costed more than I hoped for. Yatchy can still fully function without the lp core, just the battery life will be worse. I will share the source code partially to all yatchy users (also from other potential sellers) to allow to modify it, like the font used. Then I will simply compile it for them.

## The source files
In the kicad folder inside this directory are the design files for the yatchy v1, not yet tested

they (PCB's) are also a bit messy, not fully profesionall, but they do the job, I hope, as I will order them

Yes, the source files reference watchy in a few places, that's because the name "Yatchy" came after, at first this project was planned to be a small watchy fork.

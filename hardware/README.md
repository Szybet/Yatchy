# Other related docs:
1. [Power consumption](https://github.com/Szybet/Yatchy/tree/main/hardware/power-consumption)
2. [Time accuracy](https://github.com/Szybet/Yatchy/blob/main/hardware/time-accuracy.md)
3. [Creating your own](https://github.com/Szybet/Yatchy/blob/main/hardware/creating-your-own.md)

## License
<sub>Read all of it :)</sub>

This work is licensed under a [Creative Commons (4.0 International License) Attribution—Noncommercial—Share Alike](https://creativecommons.org/licenses/by-nc-sa/4.0/)

Don't scream "IT'S NOT OPEN SOURCE", let me explain myself.

- First, this work is not a watchy fork, it's a complete redesign obviously inspired by the watchy form factor, but nothing more.
- I don't want SQFMI to steal this design and sell it. Not even if I could profit of it. His bad software, bad communication, sometimes bad and questionable hardware decisions have killed so much potential of the watchy.
- I don't want someone to just make money off it and give nothing back to the project. With that said, calm down, more on that in the commercial section below.
- I don't want for someone to rename it and take the credit for the hard work.

Is it too much to ask for? The whole issue above I have is very known in the open source world, you can see it everywhere around us, here is [Jeff](https://youtu.be/4aaF2HgTVe8?t=84) talking about it. If there appears a open source license which fixes this issue, I will switch to it. 

Oh did I mention **it's is open source anyway?\*\*\*** I share the full source, the schematic, the PCB source files, the needed* (more on it below) software. It's just not the pure open source definition, but the literal interpretation of it. Element of open source like contributing, forking etc. are still allowed and welcome, but with the restrictions above.

<details>
  <summary>Further thoughts about it:</summary>

On https://opensource.org/osd we can see, in the first point "Free Redistribution" and well, first "as a component" hardware can't be a component really, so this point doesn't apply to hardware. The whole point doesn't work for hardware, we know how the world works, it works for software, that's why [InkWatchy](https://github.com/Szybet/InkWatchy) is GPL3. Let's search further, on https://www.oshwa.org/faq/#what-license-to-use we can see the question "Won’t people rip me off?" exactly what I have been searching for! The answer is "Maybe"... ugh... Well their explanation is good but a maybe is too much for me... There is also that: https://mifactori.de/non-commercial-is-not-open-source/

The blog post just shows a general opinion / statement on the internet about this topic, some of the points there make sense to me but some not at all, like `Yes, you do. But so does every patent` Wtf even is this? Sharing something to file a patent is completly different from sharing everything to be reproductible. Also patents can't be compared to hardware things most often because patents say you created something new, but hardware is often a combination of other creations, not entirely new things. Open source is not just "It's commercial, you can ripp me off", it's more than that...

so the entire situation is a joke for me. I can't have a "open source" hardware license without someone "Maybe" ripping me off. So it looks like that:

What I want from "Open source":
- To allow people modify, build their own from scratch, even sell the hardware to some extend, do deep repairs etc. (Which I do, but I'm forced to use the `Creative Commons (4.0 International License) Attribution—Noncommercial—Share Alike` license.)

What I don't want:
- People ripping me off

Sad, it just looks like someone long time ago took a software license, applied it to hardware which doesn't work in the real world and started calling out everyone "Uh oh, it's actually not open source because you are not allowing people to ripp you off"

Rant end, here is what you are left with:

A device that is not open source from the strict definition, but has the bare source for replication available, with elements of open source allowed and the author is open about this all.

* * *

It's not only me having this opinion, other makers also share it:
- https://github.com/o7-machinehum/flipper-blackhat-a33#license

:)

</details>

## The design of the Yatchy
Yatchy was designed to:
- Be created by yourself. That means it's adjusted to be hand solderable. Sooo that means changed footprints for easier hand soldering. If you can solder SMD without a problem and QFN packets aren't scary for you, but BGA is a bit, then it's for you
- Extremely hackable and modular. The main point is the module and all exposed pads for it
- Optimisation of the ammount of different components, so cheaper
- I tried to use widely available IC's and components. For example, while using some highly mauser only specific IC could improve power consumption a small bit, I preffered the simplicity and availability of the current design

Those design choices have their own flaws:
- It's not possible to order a assembled yatchy from a manufacturer. Not only because of the module, but the modified footprints have not adjusted solder mask (And I don't have the experience and money to test it). **Also to be clear, a stencil is also untested, probably won't work for the reasons explained a sentence ago, you have been warned.** - And with that in mind, let's go to the commercial section

## Commercial
As mentioned above, It's not possible to manufacture watchy on a mass scale. I don't even want to, I'm a student, I won't just start a company. I plan to sell a few Yatchy to community members and that's it.

But if you plan to create a Yatchy for yourself (because you have the skills required) you should consider creating some for people who can't do that. I will fully support it! Just remember to contribute some docs, code, help, donation and it will be fine :D

So "Commercial use" for the yatchy is using it as a cash grab, without giving anything back, which is not allowed.

If you plan to really mass produce them, then we should talk I ques.

**So you want to create your own? There is a doc for that: [Creating your own](https://github.com/Szybet/Yatchy/blob/main/hardware/creating-your-own.md)**

## The ability to enforce the license
First some explanation:

Yatchy uses esp32c6 which has a lpcore which allows for updating time on the screen in a more efficient way, in a seperate program.

So all this talking is cool but it's just on paper, I can't enforce it, let's be real. That's why I will lock the full source code of the Yatchy lp program until I make back of what I invested. It costed more than I hoped for. Yatchy can still fully function without the lp core, just the battery life will be worse. I will share the source code partially to all yatchy users (also from other potential sellers) to allow to modify it, like the font used. Then I will simply compile it for them.

<details>
  <summary>Another, more technical explanation</summary>

The ESP32-C6 in Yatchy has an LP core, which is a coprocessor optimized for efficiency and runs in RTC memory. It has a maximum of 16KB of memory (without any allocation) or just 8KB for easy compatibility with Watchy firmware, which uses the other 8KB.  

I wrote a program in Rust for this core that displays the time on the screen. This required setting up or modifying:  
- Getting Rust to run on it  
- Log mechanics (to monitor what's happening, disabled in release)  
- RTC memory handling (basic communication with the HP core, i.e., the main program)  
- RTC register access to retrieve time  
- Software SPI (since there's no native SPI)  
- Display driver  
- Minimalist font rendering via partial updates with a small buffer  
- HPCore wakeup  

All of this fits in just 8KB (technically 7KB).  

It was quite a bit of work, and no one else has done it before. However, for reasons I outlined in my repo—mainly to prevent others from copying the design and selling it without giving anything back—I plan to release only the font rendering part. For example:  

```rust
pub fn draw_zero(fb: &mut wepd::Framebuffer<NUMBER_WIDTH, NUMBER_HEIGHT>) {
    let start = Point::new(6, 4);
    let right_top = Point::new(26, 4);
    let right_t = Point::new(29, 10);
    let right_b = Point::new(29, 38);
    let right_bottom = Point::new(26, 44);
    let left_bottom = Point::new(6, 44);
    let left = Point::new(3, 38);
    let left_top = Point::new(3, 10);

    let points = [
        start,
        right_top,
        right_t,
        right_b,
        right_bottom,
        left_bottom,
        left,
        left_top,
        start,
    ];
    draw_lines(&points, fb);
}
```

You can modify it as you like, and tweaking some constants at the top will allow further adjustments, like changing letter positions.  

Oh, and once I recover the money I invested in developing Yatchy, I'll release everything publicly—no questions asked.
  
</details>

## The source files
Kicad version 8.0.6 is suggested.

In the kicad folder inside this directory are the design files for the yatchy v1, not yet tested

they (PCB's) are also a bit messy, not fully profesionall, but they do the job, I hope, as I will order them

the read-only folder contains the schematic in pdf form. It may be outdated.

Yes, the source files reference watchy in a few places, that's because the name "Yatchy" came after, at first this project was planned to be a small watchy fork.

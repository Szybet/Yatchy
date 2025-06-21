## A more technical explanation

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

All of this fits in ~~just 8KB (technically 7KB).~~ Now in 9KB (For better fonts)

It was quite a bit of work, and no one else has done it before. However, for reasons I outlined in my repo—mainly to prevent others from copying the design and selling it without giving anything back—I plan to release only the font rendering part.

Oh, and once I recover the money I invested in developing Yatchy, I'll release everything publicly—no questions asked.

## Font

Because of size constaints, the font is simply some points connecting in lines, like that:

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

But thanks to a [contribution from someone](https://github.com/themkoi) there is now svg to code above automatic conversion, so there are many fonts and they are easier to modify and add. They also look better now, as someone tinkered more with them.

### Creating your own font
(for your own watchface, or simply to adjust for inkfield)

The svg files are up to request, but there are some thing to keep in mind while modifying them:

TODO

***

Let's showcase the ones that are there now, keep in mind:
- Those are codenames and probably outdated codenames
- They are all based on one design, only with minor changes

### Font2
![image](https://github.com/user-attachments/assets/e1a06575-9901-4a02-89a7-eb5696f6194a)

### Font2_bold
![image](https://github.com/user-attachments/assets/adf56b09-64ff-491d-879b-2235d7d080e3)

### Font_szybet
![image](https://github.com/user-attachments/assets/ad64ab2e-1bd7-4fe0-9776-cef1f45c6ea0)

### Font2_opt
![image](https://github.com/user-attachments/assets/f8c7f952-6fd8-48e0-b8cf-9f404dd33e11)


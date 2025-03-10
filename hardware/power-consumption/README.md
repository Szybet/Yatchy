Once again, **once again** and ***once again and again*** those measurements are not in lab quality - I measured things again after a few months and it showed 3 times more. Did the humidity change? Did saturn catch mercury in a 10 degrees of happiness? Did the meassuring tool catch some dust? Idk? **What I can say I just used it for a while and those results are reasonably mirrored in real life, so take this doc as a best case scenario**. Also did I mention using different components from different batches can result in worse battery life from nowhere? That's also fun, so your Yatchy can work worse. Or better :)

Ok, first, measuring power at such low values is extremely hard, temperature, humidity can affect the values. Not only that, your touch, sight, planet position, random things can affect the results. What I'm trying to say, it's extremely hard, those are not 100% scientific values. But I took them many times to make sure they are similar and they have been taken on the nordic power profiler kit II, so it's valid enough I think.

The best example of problems I'm facing is that I used some nail polish to make it more water resistant, this improved the power consumption by -20uA. It could be that, my best ques at least... It's really random and hard

Some important things first:
- Yatchy obviously uses the lp core
- Those are only during time updates, not interacting with the watch. Any interaction will obviously drain a lot of battery, so the final battery consumption will be heavily based how much you just look at the time or just interact with the watch
- The `.doc` files in this dir can be used to calculate the power consumption for your settings, battery. But to understand what is going on there, first you need to read the rest of this document. **Oh and the docs screenshoots are at the bottom of this document if you don't want to change anything but you are too lazy to look it up**
- Those files are chaotic, I know, I don't care, they do their job
- For watchyv2-calculations.ods to match this: https://github.com/Szybet/InkWatchy/wiki/InkWatchy-battery-life-measurements you need to take into account that the measurments there had a night delay, while those calculations don't. To make those results match, set the wakeups in one hour to 43. Yatchy doesn't need such optimisations, that's why I skip them ;)

Devices used:
- Yatchy v1 without an accelerometer. If you want to "simulate" adding of the acc, add a few uA to the full sleep value
- Watchy v2, regular one.

Description of functioning of those devices

Yatchy:
- "Regular lp core time update" Is the most regular time update, it happens most often, almost every minute
- "First time update lp core" Happens after "hp core update"
- "hp core update" Is when the main program, in this case InkWatchy is launched. You can control how often this happens, at default it's every 29 minutes so 2 times an hour. It takes the most amount of battery, but it's important as it updates other things on the watch, not only the time.

Watchy:
- Here it's just "hp core update" all the time, as Watchy v2 (or even v3) can't use the lp/ulp core to update the time

# Conclusions
I'm happy with it

Now some images that explain the values in the docs

# Yatchy
Full sleep
![image](https://github.com/user-attachments/assets/ff78eaad-d287-4639-84dc-20dbe2035dbc)

Lp core regular time update
![image](https://github.com/user-attachments/assets/cb023cbb-6f17-4412-8198-11fd53870f8d)

Lp core first time update
![image](https://github.com/user-attachments/assets/a6857df6-64bd-4416-a049-45af35411d41)

Hp core wakeup (In the future, it could be 1s smaller)
![image](https://github.com/user-attachments/assets/0e05d273-ec75-4e86-9856-9d4da2074eb5)

# Watchy v2
Full sleep
![image](https://github.com/user-attachments/assets/3913d16c-eb4e-42a6-bcf0-e9085811ddfd)
Umm this looks a bit much, well, power consumption magic, maybe my connections is somewhere off, maybe something something is off. Previous reads, which I don't have saved gived me reads of around 130-160uA so let's treat it like this, this picture is anomaly

Hp core wakeup (It matches in timing the one from Yatchy & 
![image](https://github.com/user-attachments/assets/a7943368-8797-48e2-aaf5-8506cdde526c)

## SS
yatchy calculations:
![image](https://github.com/user-attachments/assets/ef2ed400-6937-4ef2-9c66-0ff1e4b3c7ff)
watchy calculations:
![image](https://github.com/user-attachments/assets/e429c763-7090-4ff3-b6dc-d07b69283999)


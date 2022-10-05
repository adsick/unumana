# Patterns
burst press - quick press of 
> I've played a 2d mini strategy game called "KINGDOM classic", it is available at Steam for no cost, runs on Linux, Windows and Mac (but only old versions of MacOS because no support for 32bit apps XDDD) and it features pretty minimal but clever controls: you walk sideways holding left or right arrow, you can run holding Shift, but you can also quickly tap&hold arrow button and the horse will run. 

ab hold - p(a), hold(b), this can work for two sides (ba) and can be used for shit like 4-way (exclusive) movement where J is down, K is up, JK is right and KJ is left.

short JK would move just one symbol right, JK_ (hold) would start moving it right with some fixed (or accelerated) rate. JKJ and KJK also can be used for something like moving to the start/end of the line and making it even more crazy: JKJK/KJKJ to move to the top/bottom of the doc. (note, I'm using jk here because it is more common, vim, qwerty, you know, but in practice I use dvorak and it is HT for me)

> here I also realized that I may need to support "virtual" key repeat, but I'm not sure if this a good idea. I mean normal, OS, key repeat is 90% of time sucks, but I am rather asking myself is it better to have to emit more events from the controller, or to introduce events like `MoveDownStart` and `MoveDownStop` and handle them appropriately.
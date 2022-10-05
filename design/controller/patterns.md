# General
press - only a press event
period - 
comma - 
pause - a period of time
release - only release


short press - (press(K), pause(), release)

long press

cascade - "directional" consecutive press of 2 or more keys, t(p(Kn), p(Kn+1)) > tc
I named it 'cascade' because when you look at bar diagram the bars will overlap,
but not too much.

chord - simultaneous press of 2 or more keys. t(p(Kn), p(Kn+1)) <= tc

# Combinations

with()
and([pat1, pat2]) - two patterns activated at the "same" time... hmm
or([pat1, pat2]) - 
chain([pat1, pat2]) - this will trigger if pat2 has been detected after pat1.

end(pat) - 

# Extended

decay - 

hold roll - more restricted version of roll, presses are the same, but releases are chorded.
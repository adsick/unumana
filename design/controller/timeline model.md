
... -> input events -> controller -> controller events -> ...

## Keyboard shortcuts
keyboard shortcuts are input patterns, sequences of input events
that can be used to trigger certain commands or perform actions like changing modes.

keyboard shortcut high level definition describes the way keys should be pressed for this shortcut to be performed.
this high level definition then is broken down into machine-friendly rules that match sequences of input events determining if ks has been triggered (a) can be triggered from current state (b).

(b) is needed to reduce the amount of matching.
sequence can consist of a single event.

## Binds and bounds
bind - mapping from keyboard shortcut to command(s) that it triggers.

example: 
bind(S(K1), C1) - bind short press of key K1 to command C1.
bind(L(K2), C2) - bind long press of key K2 to command C2.


bind()

bound - point in time which separates different binds

## Timeline
state of controller can be stored in the form of a sequence of keyboard events:
begin(0.0)
press(1.2, 'A')
press(1.4, 'B')
release(1.7, 'B')
release(1.8, 'A')
end(2.0)

in reality it would be more flat like Event{type: press, key: 'A', time: 1.2},
but in design I'm going to use p(K) and r(K) notation (p for press and r for release)

...
t: p(K)
t+dt: r(K)
...

dt is the duration of the key press.
if dt > ts then L(K) else S(K).

ts - "time short", duration of a regular short key press plus some margin.
usually ts is around 300 milliseconds depending on keyboard and typing speed. 

## Dependencies

let's assume that we have S(K1) - C1 and L(K1) - C2

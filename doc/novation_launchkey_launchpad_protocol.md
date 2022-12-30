# Decoding Novation Launchkey and launchpad programming SYSexes

Turns out Novation didn't document those codes. I suppose they reserve the right to change them.
For now, they seem pretty stable and there aren't many firmware updates anyway.
## Using wireshark
```shell
sudo modprobe usbmon
lsusb 
# Check the bus and device number you will then use usbmon<busnumber>
# And the device will be named <busnumber>.<device>.1 or .2 
# The launchpad pro has three of them
```
We can also nicely export packets if we do usbmon on the specific port:
```lua
((usb.src == "5.72.2") || (usb.dst == "5.72.2") || (usb.src == "5.72.1") || (usb.dst == "5.72.1")) && !(usb.transfer_type == 0x01)
```

But in practice wireshark can decode sysex packets so just use
```lua
sysex
```

When dumping the packet look into The lates USB Midi Event packet, choose message fragments and reassembled data.
That's your sysex message.
## Common to all devices I tested
### Command when starting
```hexdump
// Device inquiry
f0 
7e 7f 06 01 
f7
```

This is documented in the programmers guide.

It replies
```hexdump
f0 
7e 00 06 02 00 20 29 23 01 00 00 00 04 06 05 ; launchpad pro mk3
7e 00 06 02 00 20 29 37 01 00 00 00 02 01 07 ; launchkey
;                    ^^ ^^          ^^ ^^
;                    || ||          ++-++-++- App or bootloader version here build 2.1.7
;                    || ++------------------- 01 in app mode ; 11 in bootloader mode
;                    ++---------------------- Device type
f7

```

Dev types of MK3s:
0x23: Launchpad pro MK3
0x34: Launchkey 25
0x35: Launchkey 37
0x36: Launchkey 49
0x37: Launchkey 61
0x40: Launchkey 88

## Launchkey 61
```hexdump
; prelude
f0 00 20 29 02 0f  ;; seemingly on the Launchkey 88 the end byte is 0x12
; function and setting name (only one line like that)
05 00 45 00 06 00 1a 04 40 20 07 4d 50 43 20 4f 6e 65
05 00 45 01 05 00 1a 04 40 20 10 2a 4e 65 77 20 437573746f6d204d6f6465 ; Pad mode 1
05 00 45 00 06 00 1a 04 40 20 10 30 31 32 33 34 35 36 37 38 39 30 41 42 43 44 45
;     ^^  ^ MO    ^^          ^^ Length of name
;     ||  | ^^    ||             [The name I did set: 0123456789ABCDE          ]
;     ||  | ||    ++------------ Color of buttons when on
;     ||  | ++------------------ Pots: 06 is Mode 1, 07 is Mode 2, 08 is mode 3, 09 is mode 4 (pads start at 05...)
;     ||  +--------------------- 1 for pads, 3 for faders buttons
;     ++------------------------ 45 program ; 40 send dump (+ 2 address bytes); 10 dump (see section below)
;; programming (as many as needed but each mode has to be sent separately
  47 28 0c 6c 00 02 00 00 12 ;; keystroke shift-O 
;                ^^       ^^
;                ||       ++----------- key? likely scancodes or something of the sort
;                ++-------------------- shift is 0x02, alt is 0x04 ctrl is unknown 
  48 28 01 24 00 01 44 00 40 51    ;; Note
  49 50 02 00 00 01 40 00 47 7f 00 ;; Fader with CC
  49 38 02 00 00 01 40 00 14 7d 01 ;; CC
  49 78 02 00 00 01 40 00 40 7f 00 ;; pedal
  48 00 01 2f 00 01 40 00 00 45    ;; Pad 0
  48 00 01 2f 00 01 44 00 64 45    ;; Pad 0// fixed velocity 100
  49 01 02 2f 00 01 40 00 2d 7f 00 ;; Pad 1 with CC
; ^^ ON ^^ ^^    PC G?    CC MX MN <--- minimum (here 1) or program (not here in note) or off-value
; || ^^ || ||    ^^ ^^    ^^ ^^
; || || || ||    || ||    || ++-------- maximum (here 126) or note or program or on-value
; || || || ||    || ||    ++----------- controler change (here 20) or velocity
; || || || ||    || |+----------------- This is 4 in note mode 0 everywhere else?
; || || || ||    || +------------------ 4 only if global channel
; || || || ||    |+-------------------- Global channel if 1 (with G=4), 16 is f, 1 is 0, etc
; || || || ||    +--------------------- pickup: 0 global, 1 pickup, 2 no pickup) / Momentary 0 ; Toggle 2
; || || || ++-------------------------- color (02 on arm/select)
; || || ++----------------------------- 01 Note ; 02 CC ; 03 Program change 
; || ++-------------------------------- Object number (see table below)          
; ++----------------------------------- Message length ? 48: Note ; 49: CC or PC
```

In program change mode both min and max are set to the program.


Object numbers

|     | Pots | Faders      | Buttons below faders | Pedal | Pad up | Pad down |
|-----|------|-------------|----------------------|-------|--------|----------|
| 1   | 38   | 50          | 28                   | 78    | 00     | 08       |
| 2   | 39   | 51          | 29                   |       | 01     | 09       |
| 3   | 3a   | 52          | 2a                   |       | 02     | 0a       |
| 4   | 3b   | 53          | 2b                   |       | 03     | 0b       |
| 5   | 3c   | 54          | 2c                   |       | 04     | 0c       |
| 6   | 3d   | 55          | 2d                   |       | 05     | 0d       |
| 7   | 3e   | 56          | 2e                   |       | 06     | 0e       |
| 8   | 3f   | 57          | 2f                   |       | 07     | 0f       |
| 9   |     | 58 (master) | 30 Arm/select        |       |        |          |
|     |      |             |                      |       |

Names and colors of modes are not kept, that's only for the web interface it seems

### Asking the Launchkey to dump
Asking for pots 1
```hexdump
; prelude
f0 00 20 29 02 0f
 
05 00 40 00 06

f7
```

The keyboard replies
```hexdump
f0 00 20 29 02 0f
05 00 10 00 06 20 00 21 00 // 21 may be the internal name that is never sent by the web ui?
4938020000104800147e01  // Why is that one different??? Is that compression where they keep the buffer as is?
4839020000004800157f
483a020000004800167f
483b020000004800177f
483c020000004800187f
483d020000004800197f
483e0200000048001a7f
483f0200000048001b7f
06 00 04 40 // What is that???
// We see a 04 40 in the programming command as well
f7
```

Asking for pads 1 (so really classical)
f0 00 20 29 02 0f 
05 00 40 01 05
f7 

```hexdump
f0002029020f
05 00 10 01 05 20 10 2a 4e 65 77 20 43 75 73 74 6f 6d 20 4d 6f 64 65 21 00 ; this one has a name New Custom Mode
4800022f000040002c7f
4801022f000040002d7f
4802022f000040002e7f
4803022f000040002f7f
4804022f00004000307f
4805022f00004000317f
4806022f00004000327f
4807022f00004000337f
4808022300004000247f
4809022300004000257f
480a022300004000267f
480b022300004000277f
480c022300004000287f
480d022300004000297f
480e0223000040002a7f
480f0223000040002b7f
00 1a // Different from what it said
06 00  // Some kind of checksum?
07 00 // Similar to pots but here we have 07 00
04 40  
f7
```


### Special buttons

Channel 16

Track left CC 0x66
Track right CC 0x67
These three only work in drum mode:
Up arrow: CC 0x6a  (pressed 0xff released 0x00)
Down arrow: CC 0x6b
Right arrow: CC 0x68
Stop/Solo/mute: CC 0x69 (only in DAW mode)

This work all the time
Device select: CC 0x33
Device lock: CC 0x34
Capture midi: CC 0x4a
Quantise: CC 0x4b
Click: CC 0x4c
Undo: CC 0x4d
Play: CC 0x73
Stop: CC 0x74
Record: CC 0x75
loop: CC 0x76

Navigation mode with [...]
up: key left
left: key up
down: key down
right: key right
enter: key enter

## Launchpad pro MK3
The good thing is that the device only has 8 custom modes that all have the same structure, so much simpler than the launchkey (almost).

I just wish we could configure those devices a bit more.
There is a firmware published with partial source for older devices but not the MK3: https://github.com/dvhdr/launchpad-pro 

There is also an attempt at using the DAW mode in PureData, but it is abandoned as well https://github.com/Focusrite-Novation/r_cycle

The good thing is that they have a layout of the button codes for the DAW mode so I'll copy that here if I ever need it.

|     | 0   | 1  |  2 |  3 |  4 |  5 |  6 |  7 |  8 | 9   |
|-----|-----|----|----|----|----|----|----|----|----|-----|
| 0   | 90  | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99? |
| 1   | 80  | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89  |
| 2   | 70  | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79  |
| 3   | 60  | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69  |
| 4   | 50  | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59  |
| 5   | 40  | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49  |
| 6   | 30  | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39  |
| 7   | 20  | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29  |
| 8   | 10  | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19  |
| 9   |     |101 |102 |103 |104 |105 |106 |107 |108 |     |
| 10  | 0   |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 |     |

The 99 is probably just for the LED control, I don't think the logo is a button

### Prelude
```hexdump
; prelude
f0 00 20 29 02 0e
```
So the last byte is different from the launchkeys
### Asking for a dump of mode 1
On mode 1

```hexdump
f0
00 20 29 02 0e
05 04 
f7
```

Mode 8 is `0x05 0x0b` so they are in order. Good!~

Let's make a mode with all the features so we can have everything in one.

Had to use Firefox even if webmidi doesn't work because their UI doesn't work on chrome Linux.

Description
column 1: vertical unipolar fader CH 1 CC 42
column 2: vertical bipolar fader CH 3 CC 43
column 5 row 1: drum grid color orange start note C octave 1 CH 1
column 5 row 5: drum grid color red start note D octave 3 CH 3
column 3 row 1: midi note momentary note C oct 3 CH 1
column 4 row 1: midi note toggle note D oct 4 CH 3
column 3 row 2: program number 0 ch 1
column 4 row 2: program number 10 ch 3
column 3 row 3: CC button momentary CC 0 on 127 off 0 channel 1
column 4 row 3: CC button toggle CC 32 on 64 off 16 channel 4

I'll dump the colors in another program


We can save those as sysex now that we know how all of that works.
```shell
hexdump -C /tmp/progx.syx | cut -c 10-60 | sed s/\ \ /\ /g
```


```hexdump
f0 
00 20 29 02 0e ; prelude
; Set the name to custom mode and tell it to program?
; We will need to reverse how it sends to different modes
05 01 7f 2a 4e 65 77 20 43 75 73 74 6f 6d 20 4d 6f 64 65
 00 00 04 37 58 04 0f 
30 7f 00 00 15 53 00 01 3c 00 00 00 05 
54 02 01 4a 01 00 00 05 
55 00 01 30 00 00 00 54 
56 00 01 31 00 00 00 54 
57 00 01 32 00 00 00 54 
58 00 01 33 00 00 00 54 
49 00 03 00 02 00 00 05 
4a 02 03 0a 02 00 00 05 
4b 00 01 2c 00 00 00 54 
4c 00 01 2d 00 00 00 54 
4d 00 01 2e 00 00 00 54 
4e 00 01 2f 00 00 00 54 
3f 00 02 00 00 7f 00 05 
40 03 02 20 01 40 10 05 
41 00 01 28 00 00 00 54 
42 00 01 29 00 00 00 54 
43 00 01 2a 00 00 00 54 
44 00 01 2b 00 00 00 54 
35 7f 00 00 00 00 00 00 
36 7f 00 00 00 00 00 00
37 00 01 24 00 00 00 54 
38 00 01 25 00 00 00 54 
39 00 01 26 00 00 00 54 
3a 00 01 27 00 00 00 54 
2b 7f 00 00 00 00 00 00 
2c 7f 00 00 00 00 00 00 
2d 02 01 4a 00 00 00 05 
2e 02 01 4b 00 00 00 05 
2f 02 01 4c 00 00 00 05 
30 02 01 4d 00 00 00 05 
21 7f 00 00 00 00 00 00 
22 7f 00 00 00 00 00 00 
23 02 01 46 00 00 00 05 
24 02 01 47 00 00 00 05 
25 02 01 48 00 00 00 05 
26 02 01 49 00 00 00 05 
17 7f 00 00 00 00 00 00 
18 7f 00 00 00 00 00 00 
19 02 01 42 00 00 00 05 
1a 02 01 43 00 00 00 05 
1b 02 01 44 00 00 00 05 
1c 02 01 45 00 00 00 05 
0d 7f 00 00 00 00 00 00 
0e 7f 00 00 00 00 00 00 
0f 02 01 3e 00 00 00 05 
10 02 01 3f 00 00 00 05 
11 02 01 40 00 00 00 05 
12 02 01 41 00 00 00 05 

// That's something more, is it to say things about our program
// so the UI can reconstitute?
 00 00 00 2a 00 7f 00 15 
;^^ ^^    ^^ ^^ ^^ ^^ ^^
;|| ||    || || || || ++ color
;|| ||    || || || ++--- minimum (cannot be set on UI)
;|| ||    || || ++------ maximum (cannot be set on UI)
;|| ||    || ++--------- 0 v fader ; 1 v bipolar ; 2 v fad; 3 v bipolar
;|| ||    ++------------ CC
;|| ++------------------ Channel
;++--------------------- index (column or row)
 01 02 00 2b 01 7f 00 15


// Weirdly we only have faders for that, looks like drumpads are different?

// When we have only a fader on channel 7 we have
 00 00 00 07 00 7f 00 15
;^^ ^^    ^^
;|| ||    ++------------ CC Number
;|| ++------------------ Channel 0 is 1, etc device channel is 7f
;++--------------------- index (column or row)
f7
```

That's an empty mode. This sound really easy looks like we are just programming every pad individually.
```hexdump
f0 
00 20 29 02 0e
 05 01 7f 2a 4e 65 77 20 43 75 73 74 6f 6d 20 4d 6f 64 65
;^^       ^^
;||       ++----------------------------------------------- Size of message
;++-------------------------------------------------------- Program
;
          00 00 7f 00 00 15 ; Not sure what that is
 00 00 04 33 54 7f 00 00 05 ; If drum on row 1 col 1
 00 00 09 51 00 7f 00 00 05 ; If scale keyboard C3 minor on row 1
 00 00 08 47 58 7f 00 00 05 ;If keyboard on row 1 (and color different)
 00 00 08 3d 4e 7f 00 00 05 ;If keyboard on row 2 (and color different)
 00 00 08 33 44 7f 00 00 05 ;If keyboard on row 3 (and color different)
 
;      ^^ ^^ ^^          ^^
;      || || ||          ++- On color
;      || || ++------------- Top right of keyboard (or type of scale)
;      || ++---------------- Bottom left pad of keyboard
;      ++------------------- 8 for chromatic, 9 for scale, 4 for drum

00 00 04 33 54 09 15 0f 7f 00 00 05 ; drum pad and scale on pad 0x15
      [drum  ] [ scale]
; Types of scales
; 0 minor
; 1 major
; 2 dorian
; 3 phrygian
; 4 mixolydian
; 5 melodic minor (asc)
; 6 harmonic minor
; 7 bebop dorian
; 8 blues
; 9 minor pentatonic
; a hungarian minor
; b ukranian dorian
; c marva
; d todi
; e whole tone
; f hirajoshi

; Keys will not appear if they have a mode set on them?
 51 7f 00 00 00 00 00 00
 52 7f 00 00 00 00 00 00
;^^
;++--------------------- Key code (same as in the table above but plus 1?) 
 53 7f 00 00 00 00 00 00 
 54 7f 00 00 00 00 00 00 
 55 7f 00 00 00 00 00 00 
 56 7f 00 00 00 00 00 00 
 57 7f 00 00 00 00 00 00 
 58 7f 00 00 00 00 00 00 
 47 7f 00 00 00 00 00 00 
 48 7f 00 00 00 00 00 00 
 49 7f 00 00 00 00 00 00 
 4a 7f 00 00 00 00 00 00 
 4b 7f 00 00 00 00 00 00 
 4c 7f 00 00 00 00 00 00 
 4d 7f 00 00 00 00 00 00 
 4e 7f 00 00 00 00 00 00 
 3d 7f 00 00 00 00 00 00 
 3e 7f 00 00 00 00 00 00 
 3f 7f 00 00 00 00 00 00 
 40 7f 00 00 00 00 00 00 
 41 7f 00 00 00 00 00 00 
 42 7f 00 00 00 00 00 00 
 43 7f 00 00 00 00 00 00 
 44 7f 00 00 00 00 00 00 
 33 7f 00 00 00 00 00 00 
 33 00 01 3c 00 00 00 5e ; If keyboard base note octave 3
 33 00 03 04 02 00 00 05 ; If Program change 
 33 08 02 20 00 40 7f 05 ; If CC Moment 32 on 64 off 127 ch 9
 33 00 01 30 00 00 00 5e ; If keyboard base note octave 2 (so midi notes)
;^^ ^^ ^^ ^^ ^^ ^^ ^^ ^^
;|| || || || || || || ++- Color
;|| || || || || || ++---- Off value (0 if not)
;|| || || || || ++------- On value (0 if not)
;|| || || || ++---------- 0 momentary 1 toggle 2 trigger/Program Change
;|| || || ++------------- Base note ; Prog num
;|| || ++---------------- 1: Note ; 02 CC ; 03 Prog change 
;|| ++------------------- Channel
;++---------------------- Pad number
 34 7f 00 00 00 00 00 00 
 35 7f 00 00 00 00 00 00 
 36 7f 00 00 00 00 00 00 
 37 7f 00 00 00 00 00 00 
 38 7f 00 00 00 00 00 00 
 39 7f 00 00 00 00 00 00 
 3a 7f 00 00 00 00 00 00 
 29 7f 00 00 00 00 00 00 
 2a 7f 00 00 00 00 00 00 
 2b 7f 00 00 00 00 00 00 
 2c 7f 00 00 00 00 00 00 
 2d 7f 00 00 00 00 00 00 
 2e 7f 00 00 00 00 00 00 
 2f 7f 00 00 00 00 00 00 
 30 7f 00 00 00 00 00 00 
 1f 7f 00 00 00 00 00 00 
 20 7f 00 00 00 00 00 00 
 21 7f 00 00 00 00 00 00 
 22 7f 00 00 00 00 00 00 
 23 7f 00 00 00 00 00 00 
 24 7f 00 00 00 00 00 00 
 25 7f 00 00 00 00 00 00 
 26 7f 00 00 00 00 00 00 
 15 7f 00 00 00 00 00 00 
 16 7f 00 00 00 00 00 00 
 17 7f 00 00 00 00 00 00 
 18 7f 00 00 00 00 00 00 
 19 7f 00 00 00 00 00 00 
 1a 7f 00 00 00 00 00 00 
 1b 7f 00 00 00 00 00 00 
 1c 7f 00 00 00 00 00 00 
 0b 7f 00 00 00 00 00 00 
 0c 7f 00 00 00 00 00 00 
 0d 7f 00 00 00 00 00 00 
 0e 7f 00 00 00 00 00 00 
 0f 7f 00 00 00 00 00 00 
 10 7f 00 00 00 00 00 00 
 11 7f 00 00 00 00 00 00 
 12 7f 00 00 00 00 00 00 
f7 


```

### Other commands 
In the programmers guide we find:
```hexdump
F0 
00 20 29 02 0E 
00 xx yy 00 
F7
```
Where xx is the layout mode and yy the page.
```
00 : Session (only selectable in DAW mode)
01 : Fader
02 : Chord
03 : Custom Mode
04 : Note / Drum
05 : Scale Settings
06 : Sequencer Settings
07 : Sequencer Steps
08 : Sequencer Velocity
09 : Sequencer Pattern Settings
0A : Sequencer Probability
0B : Sequencer Mutation
0C : Sequencer Micro Step
0D : Sequencer Projects
0E : Sequencer Patterns
0F : Sequencer Tempo
10 : Sequencer Swing
11 : Programmer Mode
12 : Settings Menu
13 : Custom mode Settings
```

Available pages:
```
00-07 : for Custom Mode Views
00-03 : for any Sequencer View
00-03 : for Fader view
00-04 : for Settings Menu
00 : for any other view
```

You can get the current layout with
```
F0 
00 20 29 02 0E 
00
F7
```

Looks like the device sends also when you switch mode (here going to chord)
```
f0 
00 20 29 02 0e 
00 02 00 00 
f7
```

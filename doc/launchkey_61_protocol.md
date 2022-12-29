# Using Alsa

For now can only see replies:
F0 00 20 29 02 0F 05 00 15 02 00 F7   - Probably some kind of id

Pots of part 1 dump asked
F0 00 20 29 02 0F 05 00 10 00 06 20 07 4D 50 43 20 4F 6E 65 21 00 48 38 02 00 00 00 48 00 14 7F 48 39 02 00 00 00 48 00 15 7F 48 3A 02 00 00 00 48 00 16 7F 48 3B 02 00 00 00 48 00 17 7F 48 3C 02 00 00 00 48 00 18 7F 48 3D 02 00 00 00 48 00 19 7F 48 3E 02 00 00 00 48 00 1A 7F 48 3F 02 00 00 00 48 00 1B 7F 06 00 04 40 F7

# Using wireshark
Had to use wireshark to get data both ways but may need more decoding

Remove my mouse and keyboard and only select data transfers
((!(usb.dst == "5.4.2")) && !(usb.src == "5.4.2") && (!(usb.dst == "5.40.1")) && !(usb.src == "5.40.1")) && !(usb.transfer_type == 0x01)

Overwrite custom mode for pots (1 to 8):
CC 20, 21, 22, 23, 24, 25, 26, 27
All with:
- no name
- min 0
- max 127
- pickup global
- midi channel global

The device replied with (looks like it always say that):
```hexdump 
F0 00 20 29 02 0F 05 00 15 00 06 F7
```

Which was sent as something that looks like:
```hexdump
0000   04 f0 00 20 04 29 02 0f 04 05 00 15 07 00 06 f7
;      XX DD DD DD XX DD DD DD XX DD DD DD XX DD DD DD
```
Looks like those 04 mean that there is some data left and 07 that it is the last one?



Let's look at the full transaction:
```hexdump
# Computer to Launchkey
## Packet 1
0000   04 f0 00 20 04 29 02 0f 04 05 00 45 04 00 06 00
0010   04 1a 04 40 04 20 07 4d 04 50 43 20 04 4f 6e 65
0020   04 49 38 02 04 00 00 01 04 40 00 14 04 7f 00 49
0030   04 39 02 00 04 00 01 40 04 00 15 7f 04 00 49 3a
## Packet 2
0000   04 02 00 00 04 01 40 00 04 16 7f 00 04 49 3b 02
0010   04 00 00 01 04 40 00 17 04 7f 00 49 04 3c 02 00
0020   04 00 01 40 04 00 18 7f 04 00 49 3d 04 02 00 00
0030   04 01 40 00 04 19 7f 00 04 49 3e 02 04 00 00 01
## Packet 3
0000   04 40 00 1a 04 7f 00 49 04 3f 02 00 04 00 01 40
0010   04 00 1b 7f 06 00 f7 00
# Launchkey to computer
## Three ACK packets as URB Bulk out
## Packet 1
0000   00 43 b6 f6 6a 95 ff ff 43 03 81 30 05 00 2d 00
0010   c0 ba ab 63 00 00 00 00 ce a9 0a 00 00 00 00 00
0020   10 00 00 00 10 00 00 00 00 00 00 00 00 00 00 00
0030   00 00 00 00 00 00 00 00 04 02 00 00 00 00 00 00
0040   04 f0 00 20 04 29 02 0f 04 05 00 15 07 00 06 f7
# Computer to Launchkey
## Acknowledge
## Packet 1
0000   04 f0 00 20 04 29 02 0f 04 05 00 15 07 00 06 f7
## Launchkey acknowledge but does not reply
```
In packet 3 at the end we have a 06, does that mean that we
have 2 bytes at the end, so things are always sent by three bytes

And we would say:
04 : I have 3 bytes
07 : I have the last 3 bytes
06 : I have the last 2 bytes
so we can imagine that 
05 : I have the last byte?

We can export the data from wireshark as json see the files in protocol_dumps

We can also nicely export packets if we do usbmon on the specific port:
((usb.src == "5.72.2") || (usb.dst == "5.72.2") || (usb.src == "5.72.1") || (usb.dst == "5.72.1")) && !(usb.transfer_type == 0x01)
```hexdump
; Always the same prelude
  f0 00 20 29 02 0f 
  
  05 00 45 00 06 00 1a 04 40 20 07 4d 50 43 20 4f 6e 65 
;                          XX YY
# These are each pots:
49 38 02 00 00 01 40 00 14 7f 00
;                       CC <--- Control Change
49 39 02 00 00 01 40 00 15 7f 00
49 3a 02 00 00 01 40 00 16 7f 00
49 3b 02 00 00 01 40 00 17 7f 00
49 3c 02 00 00 01 40 00 18 7f 00
49 3d 02 00 00 01 40 00 19 7f 00
49 3e 02 00 00 01 40 00 1a 7f 00
49 3f 02 00 00 01 40 00 1b 7f 00
f7
```
Reply is
```hexdump
  f0 00 20 29 02 0f 05 00 15 00 06 f7
;                            XX YY
```


Setting a name, it doesn't seem they are stored...

```hexdump
f0 002029020f
0500450006001a044020074d5043204f6e65
49 38 02 00 00 014000147f00
4939020000014000157f00
493a020000014000167f00
493b020000014000177f00
493c020000014000187f00
493d020000014000197f00
493e0200000140001a7f00
493f0200000140001b7f00
f7
```
```hexdump
0000   f0 00 20 29 02 0f 05 00 45 00 06 00 1a 04 40 20
0010   07 4d 50 43 20 4f 6e 65 49 38 02 00 00 21 40 00
0020   14 7e 01 49 39 02 00 00 01 40 00 15 7f 00 49 3a
0030   02 00 00 01 40 00 16 7f 00 49 3b 02 00 00 01 40
0040   00 17 7f 00 49 3c 02 00 00 01 40 00 18 7f 00 49
0050   3d 02 00 00 01 40 00 19 7f 00 49 3e 02 00 00 01
0060   40 00 1a 7f 00 49 3f 02 00 00 01 40 00 1b 7f 00
0070   f7

```



Changing the minimum to 1 on pot 1 and max to 126:
```hexdump
; prelude
f0 00 20 29 02 0f  ;; seemingly on the 88 the end byte is 0x12
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

# Asking the Launchkey to dump
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

# Command when starting
```hexdump
// Device inquiry
f0 7e 7f 06 01 
f7
```

This is documented in the programmers guide.

It replies
```hexdump
f0 
7e 00 06 02 00 20 29 37 01 00 00 00 02 01 07
;                    ^^ ^^          ^^ ^^
;                    || ||          ++-++-++- App or bootloader version here build 2.1.7
;                    || ++------------------- 01 in app mode ; 11 in bootloader mode
;                    ++---------------------- Device type
f7

```

Dev types of MK3s:
0x34: Launchkey 25
0x35: Launchkey 37
0x36: Launchkey 49
0x37: Launchkey 61
0x40: Launchkey 88

# So commands are:
40 dump
45 program
10 dump result
# Special buttons

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



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


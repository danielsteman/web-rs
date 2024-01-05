# Taking ergonomics to the next level with a split ergo mech keyboard 🖖

% id: 7
% title: Taking ergonomics to the next level with a split ergo mech keyboard 🖖
% date: 2023-04-24
% tags: hardware

Already a few months ago I entered the [rabbit hole](https://www.reddit.com/r/ErgoMechKeyboards/) of custom- ergo- mechanical- split keyboards and by now I built my own. I wanted to give the [ortholinear](https://www.reviewgeek.com/70291/what-is-an-ortholinear-keyboard-and-should-you-use-one/) key layout a shot as I read many good things about it and it kind of made sense to me from an ergonomics point of view. The staggered layout of regular keyboards is a legacy from typewriters and is inferior, [some claim](https://readcaffeine.com/2022/06/ortholinear-keyboard/). Also I wanted to try a [layered key layout](https://github.com/danielsteman/ckbrd/blob/master/config/corne.keymap) to save space and make the keyboard more compact so I can bring it around easily. I also like the idea of a layered layout because it will decrease the distance between keys and raise typing speed potential.

After countless hours of research I decided to go with a wireless Corne Cherry v3. Its design is [open sourced](https://github.com/foostan/crkbd/blob/main/corne-cherry/doc/v3/buildguide_en.md) and can be freely copied. I ordered the following list of parts [here](https://kriscables.com/), except the keycaps (the last item in the list) which I order [here](https://keycapsss.com/):

- Corne Cherry V3 PCB
- nice!nano V2 Wireless Controllers
- Corne Cherry V3 Low Profile Case Acrylic/FR4
- SMD Diodes SOD-123FL
- Tactile Reset Switches
- Gateron Milky Yellow Pro Switches
- Kailh Hotswap PCB Sockets
- Rechargeable Li-Po 3.7v 110mah Batteries
- Microcontroller Hotswap Sockets
- Mill Max Socket Pins
- OEM Dye Sub Keycap Set for 40% Ortholinear Keyboards

![Parts](../images/ckbrd_packaging.jpg)

As the PCB design is open sourced you can basically order those parts at any PCB printing shop. The other parts are quite generic and widely available from other shops as well. After receiving the parts it was time to for assembly. You'll need a soldering iron and a steady hand to mount all the parts on the PCB. Without that much experience in soldering, this was quite doable, so don't let that discourage you. The Nice!Nano is really easy to flash, thanks to the [clear instructions and accompanied Github actions](https://github.com/foostan/crkbd/blob/main/corne-cherry/doc/v3/buildguide_en.md), so you don't have to bother with building the firmware yourself. [This](https://github.com/danielsteman/ckbrd) is my fork, which contains the keymap in `/config/corne.keymap`.

![Parts](../images/ckbrd_soldering.jpg)

![Parts](../images/ckbrd_switches.jpg)

![Parts](../images/ckbrd_assembled.jpg)

By now I can say that it definitely takes some time to get used to the Corne, but it definitely makes work more chill. A split keyboard makes it easier to keep a good posture for a longer time working behind a desk. In the beginning you probably want to have an open tab with the keymap to check the position of a layered key that you maybe don't use as often. Also, due to the ortholinear layout you will misfire occasionally. But that's alright. After you wrapped your brain around the new positions and keymap, you'll notice the potential it has in terms of typing speed ⚡.

# iris-rs

A WIP re-write of all the software written for Iris 16.

[Check out the original thread on SFN](https://smallformfactor.net/forum/threads/iris-16-rgb-vandal-button.1055/).

[Take a look at all Iris 16 repos](https://github.com/topics/iris-16).

## Iris Lib

A unified codebase for all functionality and datastructures related to rendering and working with
lighting effects on Iris. Implements serialization and deserialization and thus avoids having
to define the interface via Protobuf (as done by the old [iris-lib](https://github.com/iFreilicht/lib-iris))
and re-implement the strcutures and functions in TypeScript, Python and C++.

This also helps with testing, as the code running in Iris Hub is the exact same one running on the hardware.

## Iris Hub

Replaces the old [Iris Visualizer](https://github.com/iFreilicht/iris-visualizer).
It uses iris-lib by compiling it to WebAssembly, so the TypeScript logic is purely for the UI.
The rendering approach was also adapted from a canvas to modifying SVG in-place,
resulting in much sharper graphics.

The data transfer from Iris Hub to the hardware will be done directly from the browser via MIDI SysEx,
which avoids the need for users to install any software. As such, a re-write of
[Iris Manager](https://github.com/iFreilicht/iris-manager) is not necessary.

## Iris FW

The firmware for the Iris 16 hardware, utilising the [avr-hal](https://github.com/Rahix/avr-hal)
to compile for an Atmega32u4. This has not been publicly pushed yet.

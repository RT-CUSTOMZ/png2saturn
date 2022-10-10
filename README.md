# png2saturn - A tool to create single layer ctb files from png's
Written in Rust, with the intent to expose photo-resist PCBs on a Elegoo Saturn resin printer.

__Note:__ The code has many hardcoded parameters for the described scenario, but modifying it to match another printer shouldn't be to hard.
Also the assumptions made for the input are pretty tight and exception handling is really sparse right now.
So don't expect robust behavior when using it for your own purposes.

## Example
Let's assume you want to expose a single layer PCB:

If you don't already have a png of your layout ready, you need to generate one.
We use [gerbv](https://gerbv.github.io/) for this purpose.

```bash
$ gerbv -D 508 -B 0 -b '#FFFFFF' -f '#000000FF' -x png -o layout.png copper_layer.gbr
```
generates a bw png of the specified gerber.
(`-D 508` sets the dpi for our specific printer, -b / -f the background/foreground colors, -B the border).

```bash
$ png2saturn -e 90 -x 40 -y 40 layout.png layout.ctb
```
then creates a CTB file, which is ready to go on your printer.
In this case with a exposure of 90 seconds and a offset of 40 pixels from the default corner.
(I didn't test if the printer accepts a file without preview images yet, try to add them if you run into problems)

Run `$ png2saturn --help` for all available options.

## Credits
Big thanks to [cbiffle](https://github.com/cbiffle) for the epic reverse engineering of the ctb format and the provided rust library ([catibo](https://github.com/cbiffle/catibo)).

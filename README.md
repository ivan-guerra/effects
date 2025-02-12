# plasma

https://github.com/user-attachments/assets/40022551-24f7-4057-a3be-d6d0d3956f45

A graphical plasma effect visualizer with interactive controls.

This program generates animated plasma patterns in a window with real-time controls
for adjusting the visualization parameters.

## Controls

- `Space`: Cycle through color palettes
- `Left/Right`: Change pattern shape
- `Up/Down`: Adjust pattern scale
- `Escape/Q`: Exit program

## Command Line Arguments

Run the program with the `--help` flag to see the available command line
options:

```text
Options:
  -w, --width <WIDTH>      Screen width in pixels [default: 512]
  -h, --height <HEIGHT>    Screen height in pixels [default: 512]
  -s, --shape <SHAPE>      Initial plasma shape [default: ripple]
  -p, --palette <PALETTE>  Initial color palette [default: rainbow]
  -x, --scale <SCALE>      Pattern scale factor [default: 10.0]
```

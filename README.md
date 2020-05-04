## Features

- Convert .jpg and .png to Brickadia save files. 
- (Advanced) Create vertical and horizontal
- (Advanced) Customize brick size
- (Advanced) Customize brick type
- (Advanced) Customize brick materials

## Simple Usage

Install the latest version of [img2brs](https://github.com/mraware/img2brs/releases):

Drag and drop the image you want to convert onto img2brs

## Advanced Usage

Advanced usage for customizing the save requires modifying shortcut target or using the command line.

```

  Usage: img2brs [options] [filepath]

  Options:

    -v,  --vertical       false
    -s,  --size           "5 5 2"
    -b,  --brick_type     0
    -m,  --material       0
    -ra, --remove_alpha   false
    -o,  --offset         "0 0 0"

```
#### Vertical
If vertical recieves anything other than "true" it will be false.

#### Size
- Must be in the format "x y z" (with quotes).
- x and y must be a multiple of 5.
- z must be a multiple of 2.

#### Current available brick_type values
0. DefaultBrick
1. DefaultTile

#### Current available material values
0. Plastic
1. Glow
2. Metallic
3. Hologram

#### Remove Alpha (Transparent) bricks
If an image has a pixel with its alpha value < 255, a brick will not be made for it.

#### Offset
- Must be in the format "x y z" (with quotes).
- Uses microshift units.
- A 1x1 plate is 10 units wide and 4 units tall.
- A 1x1 brick is 10 units wide and 12 units tall.


## Examples

To create an image vertically.
```
C:\path\to\img2brs> img2brs --vertical true ./path/to/image.png 
```

To create an image with glowing material.
```
C:\path\to\img2brs> img2brs --material 1 true ./path/to/image.png 
```

To create an image vertically, made out of 2x2x2 tile cubes with a metallic material.
```
C:\path\to\img2brs> img2brs --vertical true --size "10 10 10" -b 1 -m 2 ./path/to/image.png 
```

This fails to load bricks because the size is incorrect. The first 2 values must be multiples of 5 and the last a multiple of 2.
```
C:\path\to\img2brs> img2brs --size "1 1 1" ./path/to/image.png 
```

## Using a Shortcut
1. Right click img2brs.exe
2. Click "Create shortcut"
3. Right click the new shortcut
4. Under "Target" you should see something along the lines of ```C:\path\to\img2brs.exe```
5. Add the options so it looks something like:
```
C:\path\to\img2brs.exe --vertical true --size "10 10 10" --brick_type 1 --material 2
```
6. Now you can drag and drop images onto the shortcut and it will use these options


## WebAssembly Usage
1. Run ```wasm-pack build``` to generate a JavaScript package
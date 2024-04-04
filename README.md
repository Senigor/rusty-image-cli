# Rusty Image CLI

Rusty Image CLI is a command-line tool written in Rust for image processing tasks. It provides various commands for common image manipulation operations such as blurring and generating fractal images.

## Installation

To use Rusty Image CLI, you need to have Rust installed on your system. You can then install Rusty Image CLI using Cargo, Rust's package manager, by running the following command:

```sh
cargo install rusty_image_cli
```

## Usage

Rusty Image CLI can be used to perform various image processing tasks via the command line. Here are the available commands:

| Command    | Description                            |
| ---------- | -------------------------------------- |
| `blur`     | Apply a blur effect to an input image. |
| `fractal`  | Generate a fractal image.              |
| `brighten` | Adjust the brightness of an image.     |
| `crop`     | Crop an image to specific dimensions.  |
| `rotate`   | Rotate an image by a specified angle.  |

## Dependencies

Rusty Image CLI relies on the following open-source libraries:

- [image-rs](https://crates.io/crates/image): A popular image processing library for Rust.

## Examples

### Blur Command

Apply a blur effect to an input image.

| Argument          | Description              |
| ----------------- | ------------------------ |
| `-i`, `--infile`  | Path to the input file.  |
| `-o`, `--outfile` | Path to the output file. |
| `-d`, `--depth`   | Depth of the blur.       |

```sh
rusty_image_cli blur -i input.png -o output.png -d 2.0
```

### Fractal Command

Generate a fractal image.

| Argument          | Description              |
| ----------------- | ------------------------ |
| `-o`, `--outfile` | Path to the output file. |

```sh
rusty_image_cli fractal --outfile fractal.png
```

### Brighten Command

Brighten or darken an image.

| Argument          | Description                                                                                         |
| ----------------- | --------------------------------------------------------------------------------------------------- |
| `-i`, `--infile`  | Path to the input file.                                                                             |
| `-o`, `--outfile` | Path to the output file.                                                                            |
| `-v`, `--value`   | Value to brighten the image. Positive numbers brighten the image, while negative numbers darken it. |

```sh
rusty_image_cli brighten -i input.png -o output.png -v 50
```

### Crop Command

Crop an image to a specified size and location.

| Argument          | Description                       |
| ----------------- | --------------------------------- |
| `-i`, `--infile`  | Path to the input file.           |
| `-o`, `--outfile` | Path to the output file.          |
| `-x`              | X-axis value (starting position). |
| `-y`              | Y-axis value (starting position). |
| `--width`         | Width value of the cropped area.  |
| `--height`        | Height value of the cropped area. |

```sh
rusty_image_cli crop -i input.png -o output.png -x 100 -y 100 --width 200 --height 200
```

### Rotate Command

Rotate an image by a specified angle.

| Argument          | Description                                            |
| ----------------- | ------------------------------------------------------ |
| `-i`, `--infile`  | Path to the input file.                                |
| `-o`, `--outfile` | Path to the output file.                               |
| `-v`, `--value`   | Rotation degrees (accepts values: "90", "180", "270"). |

```sh
rusty_image_cli rotate -i input.png -o output.png -v 90
```

### Invert Command

Invert an image.

| Argument          | Description              |
| ----------------- | ------------------------ |
| `-i`, `--infile`  | Path to the input file.  |
| `-o`, `--outfile` | Path to the output file. |

```sh
rusty_image_cli invert -i input.png -o output.png
```

### Grayscale Command

Make an image fully grey.

| Argument          | Description              |
| ----------------- | ------------------------ |
| `-i`, `--infile`  | Path to the input file.  |
| `-o`, `--outfile` | Path to the output file. |

```sh
rusty_image_cli grayscale -i input.png -o output.png
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

# Rusty Image CLI

Rusty Image CLI is a command-line tool written in Rust for image processing tasks. It provides various commands for common image manipulation operations such as blurring and generating fractal images.

## Installation

To use Rusty Image CLI, you need to have Rust installed on your system. You can then install Rusty Image CLI using Cargo, Rust's package manager, by running the following command:

```sh
cargo install rusty_image_cli
```

## Usage

Rusty Image CLI can be used to perform various image processing tasks via the command line. Here are the available commands:

| Command   | Description                            |
| --------- | -------------------------------------- |
| `blur`    | Apply a blur effect to an input image. |
| `fractal` | Generate a fractal image.              |

## Dependencies

Rusty Image CLI relies on the following open-source libraries:

- [image-rs](https://crates.io/crates/image): A popular image processing library for Rust.

## Examples

### Blur Command

To apply a blur effect to an image named `input.png` and save the blurred image as `output.png` with a blur depth of 2.0, you can use the following command:

```sh
rusty_image_cli blur --infile input.png --outfile output.png --depth 2.0
```

### Fractal Command

To generate a fractal image and save it as `fractal.png`, you can use the following command:

```sh
rusty_image_cli fractal --outfile fractal.png
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

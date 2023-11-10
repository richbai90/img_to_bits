# img_to_bits

`img_to_bits` is a command-line tool written in Rust that allows you to convert images into a binary representation.

## Features

- Convert images to bits with specified bit depth. Supported depths include 2, 4, and 8.
- Command-line interface for easy use and automation.
- Customizable output file naming.

## Usage

To use `img_to_bits`, you need to specify the input file and the output file, along with the bit depth for conversion. Supported bit depths include 2, 4, and 8.

```shell
img_to_bits --file <input_file> --output <output_file> --depth <bit_depth>
```

## Installation

**Release**
Download the pre-built binary from [releases](https://github.com/richbai90/img_to_bits/releases/latest)

**Source**

To install `img_to_bits` from source, ensure you have Rust and Cargo installed, then follow these steps:

1. Clone the repository.
2. Navigate to the repository directory.
3. Build the project using Cargo:

```shell
cargo build --release
```

The executable will be located in the `target/release` directory.

## Dependencies

- `clap` for parsing command-line arguments.
- `image` for handling image files.
- `bitflags` for bit manipulation.

## Contributing

Contributions to `img_to_bits` are welcome. Please feel free to fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/richbai90/img_to_bits/blob/main/LICENSE) file for details.

---

You can find the source code and contribute to the project using the following links:

- [Cargo.toml](https://github.com/richbai90/img_to_bits/blob/main/Cargo.toml) - Project manifest with dependency details.
- [main.rs](https://github.com/richbai90/img_to_bits/blob/main/src/main.rs) - Main source file containing the application logic.
- [program_args.rs](https://github.com/richbai90/img_to_bits/blob/main/src/program_args.rs) - Module for command-line argument definitions.

For more information on how to use the tool, refer to the comments and documentation within the [main.rs](https://github.com/richbai90/img_to_bits/blob/main/src/main.rs) and [program_args.rs](https://github.com/richbai90/img_to_bits/blob/main/src/program_args.rs) files.

[AskTheCode.ai](https://askthecode.ai) | [Documentation](https://docs.askthecode.ai) | [GitHub](https://github.com/askthecode/askthecode.github.io) | [Twitter](https://twitter.com/askthecode_ai)
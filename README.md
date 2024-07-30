# Social Parser

Library to process archive bundles from social platforms like Facebook, Instagram, Discord, etc. New features and reliability improvements are added regularly.

Rust and TypeScript library bindings are provided and can be used for anything (e.g. collect your personal social media data to LoRA finetune an LLM). Other languages may be supported in the future.

## Features

- **Support for multiple social platforms**: See API documentation for a list of supported platforms. <!-- TODO: Add link -->
- **File format handling**: Able to parse entire archive bundles at once.
- **Data normalization**: Normalizes data from different platforms into a common format.
- **Export formats**: Supports exporting data to normalized format
- **Language support**: Supports multiple common languages for data processing.
- **Logging**: Provides logging implementation for each language binding.

## Getting Started

### Installation

To install the package, run:

```bash
npm install social-parser
```
<!-- TODO: Add other binding info -->

### Usage

Refer to the [API documentation](API.md) for details on how to use the library.
<!-- TODO: Update -->

## Contributing

Contributions are always welcomed! Start by forking the repository and making a PR when follow. Sharing bugs and making feature requests are also appreciated.

See the [CONTRIBUTING](CONTRIBUTING.md) file for more information.

## FAQ

Q: Why write in Rust?
A: Rust was chosen more for its broad binding support than its memory-safe, runtime performance. When the library started in 2023, there was no better option than Rust for this use case.

Q: Why distribute the TypeScript version on jsr?
A: JSR provides a modern package registry to deploy and distrbute packages in ESM and CJS with backwards compatibility with the npm ecosystem.

## License

This project is licensed under LGPL-3.0. See the [LICENSE](LICENSE) file for details. If some project needs a different license, please contact the author.

# Social Parser

Library to process archive bundles from social platforms like Facebook, Instagram, Discord, etc. New features and reliability improvements are added regularly.

Rust and bindings for other languages are provided and can be used for anything (e.g. collect your personal social media data to LoRA finetune an LLM). Other languages may be supported in the future.

## Features

- **Support for multiple social platforms**: See API documentation for a list of supported platforms.
- **File format handling**: Able to parse entire archive bundles at once.
- **Data normalization**: Normalizes data from different platforms into a common format.
- **Export formats**: Supports exporting data to normalized format
- **Language support**: Supports multiple common languages for data processing.
- **Logging**: Provides logging implementation for each language binding.

## Getting Started

### Installation

To install the package, run:

```bash
cargo install social_parser
```

## License

This project is licensed under LGPL-3.0. See the [LICENSE](LICENSE) file for details. If some project needs a different license, please contact the author.

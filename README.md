# Asset Bundler 2

A modular Rust toolbox for media asset management. It was created to handle the tedious parts of working with large sets of images and 3D textures—converting formats, compressing files, and organizing folders—so you can focus on the creative work.

## Features

- ** Image Conversion** – Batch convert between popular formats, including full **DDS** support for game development textures.
- ** File Bundling & Merging** – Combine and organize files into structured packages.
- ** Compression** – Compress assets to save space without losing quality.
- ** Encryption** – Keep sensitive assets secure with built-in encryption.
- ** User Interface** – A growing GUI built with [Iced](https://iced.rs/) (WIP, but already usable).

*The project is under active development—new features are added if needed.*

## Tech Stack

- **Language:** Rust
- **GUI:** Iced
- **Image Processing:** `image`, `dds`, `webp`, `jpeg-encoder`, `png`, `exr`, `lcms2`, and more.
- **Compression:** `zstd`, `xz2`, `bzip2`
- **Async & Concurrency:** `tokio`, `rayon`

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable)
- Cargo (comes with Rust)
- libavif

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/xidq/asset_bundler_2.git
   cd asset_bundler_2
2. Make sure to create file for missing password (in cargo "src/pass") and have lib dependencies installed
3. Build the project:
    cargo run --release
4. Run the project:
    cargo build --release


   asset_bundler_2/
Project Structure
├── src/
│   ├── app/             # Main application logic
│   ├── binarka/         # Binary-related utilities
│   ├── dds_ops/         # DDS texture operations
│   ├── encodery/        # Encoding/decoding logic
│   ├── enumy/           # Shared enums
│   ├── file_merge/      # File merging & bundling
│   ├── image_conversion/# Image format conversion
│   ├── kompresja/       # Compression utilities
│   ├── szyfrowanie/     # Encryption module
│   └── ui_iced/         # Iced GUI implementation
├── Cargo.toml           # Workspace configuration
└── README.md            # This file

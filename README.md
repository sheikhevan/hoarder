<div align="center">
  <h1 align="center">Hoarder (v2)</h1>
  <h3>The greatest renaming utility the world has ever known</h3>
</div>

<br/>

<div align="center">
  <a href="https://github.com/sheikhevan/hoarder/stargazers"><img alt="STAR MY REPO" src="https://img.shields.io/github/stars/evfeal/hoarder"></a>
  <a href="https://github.com/sheikhevan/hoarder/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/badge/license-GPLv3-purple"></a>
</div>

<br/>

## Features

- **Full POSIX Compliance :** Supports all the things you would expect from a Unix CLI
- **Directory Mode:** Choose to organize files into directories instead of simply renaming them (i.e. YYYY-MM-DD -> YYYY/YYYY-MM-DD)
- **Images:** Rename images using exif data and the IMG_YYYY-MM-DD format!

## Tech Stack

- [Rust](https://www.rust-lang.org/)
  - [clap](https://crates.io/crates/clap)
  - [kamadak-exif](https://crates.io/crates/kamadak-exif)

## Getting Started

### Prerequisites

Here's what you need to be able to run hoarder:

- Unix-based system (Windows support coming one day)
- Rust compiler
- libexif

### 1. Clone the repository

```shell
git clone https://github.com/evfeal/hoarder.git
cd hoarder
```

### 2. Compile the program
```shell
cargo build --release
```

### 3. Run the program

```shell
./target/release/hoarder
```

### Contributors

<a href="https://github.com/mfts/papermark/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=evfeal/hoarder" />
</a>

# Open game launcher scripts

This repository contains a set of scripts used to create some required files for the Open Game Launcher like game developer online files checksum to check the integrity of the files when downloading.

## Usage

Go to the releases page to download the latest release of the script. All scripts have a Unix and Windows version.

## Scripts

### checksum

This script is used to generate a checksum file for a directory. It recursively walks the root directory and outputs a checksum file.


You can execute the script by running:

```bash
./checksum > checksum_file
```

Or you can pass a directory as an argument to generate a checksum file for that directory if you install the script globally:

```bash
./checksum /path/to/directory > checksum_file
```

## Building

To build the scripts manually you will need rust to be installed and configured in your system.

After that, you can build the scripts by running `cargo build --release` and check the `target` directory. If you want to cross compile you can install [cross](https://github.com/cross-rs/cross) and run the scripts in the `scripts` folder to generate windows and unix binaries.

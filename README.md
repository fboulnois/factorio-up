# factorio-up

A single executable to download and run the latest stable version of Factorio headless server, optimized for containers.

## Features

- Downloads the latest stable version of Factorio headless server and verifies the checksum
- Extracts the downloaded archive into a new directory
- (optional) Initializes a map with custom settings
- (optional) Symlinks the binary file and data directory
- (optional) Runs the command as a specific user
- (optional) Runs the Factorio binary with custom settings

## Usage

Options are specified as command line arguments. The basic usage is:

```sh
factorio-up [OPTIONS] ...
```

Running the command without any options will download the latest stable version of Factorio headless server and extract it into a new directory.

### Options

| Option | Description |
| ------ | ----------- |
| `--init-map <init_map>` | Initialize the map settings [default: false] |
| `--save-file <save_file>` | File path to the save .zip [default: server-default.zip] |
| `--map-gen-settings <map_gen_settings>` | File path to the map generator settings [default: map-gen-settings.json] |
| `--map-settings <map_settings>` | File path to the map settings [default: map-settings.json] |
| `--exe-path <exe_path>` | File path to symlink the downloaded server binary |
| `--data-dir <data_dir>` | Directory to symlink the downloaded server data |
| `--user <user>` | Run the command as this user |

Additional trailing options will be treated as a command with arguments to execute. For example, `factorio-up --user $USER echo hello world` will run the command `echo hello world` as the current user after downloading and extracting the Factorio server. This is useful for running Factorio or other scripts or commands after an update.

## Build

```sh
docker build . --tag factorio-up
```

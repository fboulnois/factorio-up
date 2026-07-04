# factorio-up

A single executable to download and run the latest stable version of Factorio headless server, optimized for containers.

## Features

- Downloads the latest stable version of Factorio headless server and verifies the checksum
- Extracts the downloaded archive into a versioned cache directory
- (optional) Initializes a map with custom settings
- (optional) Runs the command as a specific user
- (optional) Runs the Factorio binary with custom settings

## Usage

Options are specified as command line arguments. The basic usage is:

```sh
factorio-up [OPTIONS] ...
```

Running the command without any options will download the latest stable version of Factorio headless server and extract it into a versioned cache directory.

### Options

| Option | Description |
| ------ | ----------- |
| `--init-map` | Initialize the map settings [default: no] |
| `--save-file <save_file>` | File path to the save .zip [default: server-default.zip] |
| `--map-gen-settings <map_gen_settings>` | File path to the map generator settings [default: map-gen-settings.json] |
| `--map-settings <map_settings>` | File path to the map settings [default: map-settings.json] |
| `--user <user>` | Run the command as this user |

Additional trailing options will be treated as a command with arguments to execute. For example, `factorio-up --user $USER echo hello world` will run the command `echo hello world` as the current user after downloading and extracting the Factorio server. This is useful for running Factorio or other scripts or commands after an update.

## Build

```sh
docker build . --tag factorio-up
```

## Deploy

See the [Dockerfile](https://github.com/fboulnois/factorio-docker/blob/main/Dockerfile) for a minimal example on how to deploy the executable in a container.

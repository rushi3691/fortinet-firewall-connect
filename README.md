# Fortinet-Connect

FortinetConnect is an application designed to automate the process of connecting to Fortinet firewalls. It eliminates the need for manual Wi-Fi login and manages session expiration seamlessly.

## Features

- **Automated Connection**: FortinetConnect automatically connects to your Fortinet firewall, saving you the hassle of manual login.
- **Session Management**: The app handles session expiration, ensuring you're always connected when you need to be.
- **Auto Start**: FortinetConnect can be configured to start automatically when your system boots up.
- **Built in rust**: FortinetConnect is built in Rust, ensuring high performance and low resource usage.

## Available Builds

FortinetConnect is available for the following platforms:
(All builds are available in the [releases](https://github.com/rushi3691/fortinet-firewall-connect/releases))

| Platform              | Architecture | Available          | File              |
|-----------------------|--------------|--------------------|-------------------|
| Linux(ubuntu, debian) | x64          | :white_check_mark: | use amd64.deb     |
| macOS(universal)      | arm64, intel | :white_check_mark: | use universal.dmg |
| macOS                 | arm64        | :exclamation:      | use aarch64.dmg   |
| macOS                 | intel        | :grey_exclamation: | use x64.dmg       |
| Windows               | x64 (64 bit) | :white_check_mark: | use x64-setup.exe |
| Windows               | x86 (32 bit) | :grey_exclamation: | use x86-setup.exe |

:white_check_mark: - Available and tested  
:grey_exclamation: - Available but not tested  
:exclamation: - Use universal build if this doesn't work

## Todos
- [x] Base app with system tray and creds window
- [x] Add Logging 
- [x] Support Auto start on boot
- [x] Ensure Single instance 
- [ ] Sign the builds
- [ ] Notifications
- [ ] StrongHold


## Installation(todo)

Detailed installation instructions for each platform can be found in the [Installation Guide](INSTALLATION.md).

## Usage(todo)

Instructions on how to use FortinetConnect can be found in the [User Guide](USER_GUIDE.md).

## Contributing(todo)

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for more details.

## License(todo)

FortinetConnect is licensed under the [MIT License](LICENSE).
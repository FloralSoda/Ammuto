# Ammuto
## The Goal
Ammuto's goal is to provide users with a convenient, beautiful way to organise their files without the 
hassle that comes with existing technologies.
Designed after being unsatisfied with online file hosting solutions, Ammuto seeks to fill the gaps
and niches where these solutions don't provide.

## Features
* Providing a local environment to host your data, instead of having to set up an entire
    home server just for organisation.
* A responsive, modern UI built using Tauri/Yew and Rust.
* A plug and play solution for hosting servers, so that anyone with an internet connection can host their
     own Ammuto server.
* Giving total plugin support, as well as plugin mismatch support, such that a client or server can host 
    plugins to give them extra functionality, without requiring the plugins on the other end.
* The ability to freely read and generalise the tagging metadata in any environment using JSON5.
* Connection to multiple servers, including a fake local instance running on the desktop
* Open source and decentralised, so the same app can connect to multiple unique Ammuto servers, and the same server can be connected to by multiple unique apps

## What this repository provides
In general, this project aims to provide code for the adoption and use of Ammuto.
* A library to give an easy groundwork for an implementation of Ammuto
* A client app to enable access to the Ammuto technology, regardless of if you want to use a server or a network
* A server app to simply install Ammuto in the device of your choice and host your files on the internet
* A wiki explaining the protocols and data structures used

## Contributing
Please create a pull request to contribute. Contributions are gladly welcome

## Building
As this app is built on Rust, you will, of course, require Rust. [Follow this link](https://www.rust-lang.org/tools/install) to learn how to install Rust and its prerequisites on your operating system.

To build the graphical side of the project, you will need [Tauri's prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites).
### CLI
Building the library: `cargo build --lib ammuto-lib`.\
To run the CLI: `cargo run --bin ammuto-cli`\
To run the client: `cargo tauri dev`

### VSCode
Ensure you have the following extensions installed:
* CodeLLDB
* rust-analyzer
* Tauri

Use the launch.json configurations to build, run and test the project.

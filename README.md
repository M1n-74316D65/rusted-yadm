# Rusted-YADM (Yet Another Dotfile Manager)

Rusted-YADM is a command-line dotfile manager written in Rust. It helps you manage, version control, and synchronize your dotfiles across multiple machines.

## Features

- Clone dotfile repositories from Git (HTTPS and SSH)
- Add new dotfiles to the repository
- Commit changes to your dotfiles
- Push changes to remote repository
- Automatically copy dotfiles to your home directory after cloning

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)
- Git

### Building from source

1. Clone this repository:

    ```bash
    git clone https://github.com/M1n-74316D65/rusted-yadm.git
    cd rusted-yadm
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Install the binary:

    ```bash
    cargo install --path .
    ```

## Usage

### Clone a repository

To clone a repository, use the `clone` subcommand followed by the repository URL and the desired destination directory:

```bash
rusted-yadm clone https://github.com/M1n-74316D65/dotfiles.git
```

This will clone the repository to the specified directory and automatically copy the files to your home directory.

Force clone:

```bash
rusted-yadm clone https://github.com/M1n-74316D65/dotfiles.git -f
```

### Add a file to the repository

To add a file to the repository, use the `add` subcommand followed by the path to the file:

```bash
rusted-yadm add ~/.bashrc
```

This will add the specified file to the repository and commit the changes.

### Commit changes

To commit changes to the repository, use the `commit` subcommand followed by the commit message:

```bash
rusted-yadm commit "Initial commit"
```

This will commit the changes to the repository.

### Push changes

To push changes to the remote repository, use the `push` subcommand:

```bash
rusted-yadm push
```

This will push the changes to the remote repository.

## Contributing

Contributions are welcome! If you have any suggestions or improvements, please open an issue or submit a pull request.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for more information.

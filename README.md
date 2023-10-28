# SSHive

SSHive is a Rust-powered application that simplifies the management of SSH connections. It provides a set of features designed to make your SSH workflow efficient and hassle-free.

**Note: This project is currently under development.**

## Key Features

- Blazingly fast: SSHive is built with performance in mind, ensuring speedy and responsive SSH connections.
- Fastly switch remote servers: Easily establish SSH connections to remote servers with minimal effort.
- Configuration file: SSHive supports a configuration file to store commonly used SSH connection details, making it convenient to connect to frequently accessed servers.

## Installation

To get started with SSHive, install it with `cargo`:

```bash
$ cargo install sshive
```

## Usage

SSHive supports the following subcommands:

- `connect {TAG}`: Connects to a remote server using the specified tag. For example, `sshive connect my-server`.
- `--version` or `-V`: Displays the version of SSHive.
- `--help` or `-h`: Shows the help message and usage instructions.

## Contributing

Contributions to SSHive are welcome! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix: `git checkout -b my-feature`
3. Make your changes and commit them: `git commit -am 'Add new feature'`
4. Push the branch to your forked repository: `git push origin my-feature`
5. Open a pull request on the main SSHive repository.

Please ensure that your code adheres to the project's coding style and includes appropriate tests.

## License

This project is licensed under the [MIT License](LICENSE).
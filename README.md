# SSHive [DEPRECATED]

SSHive is a Rust-powered application that simplifies the management of SSH connections. It provides a set of features designed to make your SSH workflow efficient and hassle-free.

<span style="color:red">Notice:</span> **This project is deprecated.**

## Deprecation Notice

The sshive project has been deprecated as the functionality it provided is now available as built-in features in SSH. We recommend utilizing the native SSH features for your remote access and management needs.

## Archive

This repository will be archived and will no longer receive updates or maintenance. Feel free to fork the repository if you wish to continue development or use the existing codebase as a reference.

## Alternatives

If you are looking for alternatives to `sshive`, consider use this configuration with `ssh``:
```js
Host <alias>
  User <user>
  HostName <host>
  Port <port>
  RemoteCommand <shell>
  RequestTTY force
  IdentityFile <path>
```
#### Usage
```bash
$ ssh <alias>
```

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

## Idealogy
> The origin of the name SSHive stems from a combination of two key elements: `SSH` and `hive`.
<details>
  <summary>More</summary>

- **SSH**: SSH stands for "Secure Shell." It is a widely used network protocol that enables secure remote access and communication between two systems. SSH provides encryption, authentication, and data integrity, making it a popular choice for securely managing remote servers.
- **Hive**: The term "*hive*" typically refers to a structure or place where bees live and work together. It symbolizes collaboration, organization, and a centralized hub.
> When these two elements are combined, `SSHive` represents a concept where SSH connections are centralized, organized, and managed securely, similar to how bees collaborate and work together in a hive. The name SSHive embodies the idea of a centralized platform for managing SSH connections and emphasizes the importance of security and collaboration in remote server management.

> By choosing the name SSHive, it conveys the notion of a unified and secure environment for efficiently handling SSH connections, while also representing the cooperative nature of managing multiple remote servers in a central location.
</details>

## License

This project is licensed under the [MIT License](LICENSE).
# Bundle 

A lightweight Rust CLI tool that allows you to bundle multiple external commands into a single unified interface with custom subcommands.

## Features

- **Unified Interface**: Combine multiple CLI tools under one command
- **Custom Subcommands**: Define your own subcommand names and mappings
- **Metadata Support**: Add descriptions, author info, and about text
- **Seamless Execution**: Transparently pass arguments to underlying commands
- **Unix-friendly**: Uses `exec()` for proper process replacement
- **Nix Support**: Built with Nix for reproducible builds

## Installation

### From Source (Cargo)

```bash
# Clone and build from source
git clone 
cd bundle
cargo build --release

# Install globally
cargo install --path .
```

### Using Nix

```bash
# Run directly with nix run
nix run github:FlorianNAdam/bundle -- --help

# Build with nix build
nix build github:FlorianNAdam/bundle

# Enter development shell
nix develop github:FlorianNAdam/bundle
```

## Usage

### Basic Syntax

```bash
bundle -n <main-command> -c <mapping> [options] [--] [subcommand] [args...]
```

### Creating a Command Bundle

```bash
# Create a 'dev' command with git and docker subcommands
bundle \
  -n dev \
  -c "git:/usr/bin/git:Version control operations" \
  -c "docker:/usr/bin/docker:Container management" \
  -c "make:/usr/bin/make:Build automation" \
  --description "Development tools wrapper" \
  --author "Your Name <your.email@example.com>"
```

### Using the Generated Command

Once created, you can use your bundled command:

```bash
# Get help
dev --help

# Use git subcommand
dev git status

# Use docker subcommand  
dev docker ps

# Use make subcommand
dev make build
```

## Command Mapping Format

The `-c/--command` option uses the format:
```
name:path[:description]
```

- **name**: The subcommand name users will type
- **path**: Absolute path to the executable
- **description**: Optional help text (shown in `--help`)

### Examples

```bash
# Minimal mapping (no description)
-c "ls:/bin/ls"

# With description
-c "list:/bin/ls:List directory contents"

# Multiple mappings
-c "python:/usr/bin/python3:Python interpreter" \
-c "pip:/usr/bin/pip3:Python package manager" \
-c "venv:/usr/bin/python3 -m venv:Create virtual environments"
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--command` | `-c` | Command mapping (format: `name:path[:description]`) |
| `--name` | `-n` | Name of the main command (required) |
| `--description` | `-d` | Description for the main command |
| `--author` | `-a` | Author information |
| `--about` | `-b` | Detailed about text |
| Trailing args | - | Arguments passed to the subcommand |

## Examples

### Development Tools Bundle

```bash
bundle \
  -n devtools \
  -c "build:/usr/bin/cargo build:Build the project" \
  -c "test:/usr/bin/cargo test:Run tests" \
  -c "fmt:/usr/bin/cargo fmt:Format code" \
  -c "clippy:/usr/bin/cargo clippy:Lint code" \
  --description "Rust development tools" \
  --author "Dev Team <dev@example.com>"
```

### System Administration Bundle

```bash
bundle \
  -n sys \
  -c "disk:/bin/df -h:Show disk usage" \
  -c "memory:/usr/bin/free -h:Show memory usage" \
  -c "process:/usr/bin/htop:Process monitor" \
  -c "network:/bin/ss -tulpn:Network connections" \
  --about "System monitoring and administration tools"
```

## Nix Flake Outputs

The flake provides:

- `packages.bundle`: The built binary package
- `defaultPackage`: The main package (`bundle`)
- `devShell`: Development environment with Rust tools

### Using as a Nix Dependency

```nix
{
  inputs.bundle.url = "github:FlorianNAdam/bundle";

  outputs = { self, nixpkgs, bundle }: {
    packages.x86_64-linux.my-app = /* ... */;
    
    # Use bundle in your derivation
  };
}
```

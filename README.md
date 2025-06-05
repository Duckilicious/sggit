# Scatter Gather Git (sggit)

A Git wrapper CLI tool for managing files scattered across your operating system. Track, synchronize, and version control configuration files, scripts, and other important files from their original locations while maintaining a centralized git repository.

## Features

- **Cross-platform file tracking**: Track files from anywhere on your filesystem
- **Platform-aware synchronization**: Automatically handles different file locations per platform (Linux, macOS, Windows)
- **Automatic git commits**: The `update` command creates detailed git commits with timestamps
- **Safe bidirectional sync**: Modification date checks prevent accidental overwrites
- **JSON configuration**: Human-readable tracking configuration

## Installation

### From Source

```bash
git clone <repository-url>
cd sggit
cargo install --path .
```

### Prerequisites

- Rust 1.75+ (for compatibility with older systems)
- Git (for repository operations)

## Quick Start

1. **Initialize a new sggit repository**:
   ```bash
   mkdir my-configs
   cd my-configs
   sggit init
   ```

2. **Add files to track**:
   ```bash
   sggit add ~/.bashrc
   sggit add ~/.vimrc
   sggit add /etc/nginx/nginx.conf
   ```

3. **Update from remote locations**:
   ```bash
   sggit update
   ```

4. **Sync changes back to remote locations**:
   ```bash
   sggit sync
   ```

## Commands

### `sggit init`

Creates an empty git repository and initializes sggit configuration.

```bash
sggit init
```

### `sggit add <remote_path>`

Adds a file to be tracked by sggit. The file must exist at the specified path.

```bash
sggit add ~/.bashrc
sggit add /home/user/scripts/backup.sh
```

**What it does:**
- Records the file's remote location and current platform
- Stores metadata in `.sggit/config.json`
- Creates a local filename based on the original filename

### `sggit update`

Copies files from their remote locations to the local sggit repository and commits the changes.

```bash
sggit update
```

**What it does:**
- Copies tracked files from remote locations to local repository
- Creates a git commit with detailed information:
  - Number of files updated
  - List of files with their last modified timestamps
- Only processes files that match the current platform

**Example commit message:**
```
Update 2 files from remote locations

- bashrc (modified: 2025-01-05 14:30:25 UTC)
- vimrc (modified: 2025-01-05 14:28:10 UTC)
```

### `sggit sync`

Updates remote files with changes from the local repository.

```bash
sggit sync
```

**What it does:**
- Copies files from local repository back to their remote locations
- Checks modification dates to prevent overwriting newer remote files
- Only syncs files where local version is newer than remote version
- Updates last sync timestamp in configuration

## Configuration

Sggit stores its configuration in `.sggit/config.json`. This file tracks all managed files and their metadata.

### Configuration Format

```json
{
  "files": {
    "bashrc": [
      {
        "remote_path": "/home/user/.bashrc",
        "platform": "linux",
        "local_path": "bashrc",
        "last_synced": "2025-01-05T14:30:25.123456789Z"
      }
    ],
    "config.ini": [
      {
        "remote_path": "/home/user/.config/app/config.ini",
        "platform": "linux", 
        "local_path": "config.ini",
        "last_synced": null
      },
      {
        "remote_path": "C:\\Users\\user\\AppData\\app\\config.ini",
        "platform": "windows",
        "local_path": "config.ini", 
        "last_synced": null
      }
    ]
  }
}
```

### Platform Support

Sggit automatically detects the current platform and only processes files that match:
- `linux` - Linux systems
- `macos` - macOS systems  
- `windows` - Windows systems

This allows you to track the same logical file across different platforms with different paths.

## Use Cases

### Configuration Management
```bash
# Track shell configurations
sggit add ~/.bashrc
sggit add ~/.zshrc
sggit add ~/.tmux.conf

# Track application configs
sggit add ~/.config/nvim/init.vim
sggit add ~/.gitconfig
```

### System Administration
```bash
# Track server configurations
sggit add /etc/nginx/nginx.conf
sggit add /etc/systemd/system/myapp.service
sggit add /etc/crontab
```

### Development Environment
```bash
# Track IDE settings
sggit add ~/.vscode/settings.json
sggit add ~/.vscode/keybindings.json

# Track development tools
sggit add ~/.tool-versions
sggit add ~/.env.example
```

## Workflow Example

1. **Set up tracking**:
   ```bash
   mkdir ~/dotfiles
   cd ~/dotfiles
   sggit init
   sggit add ~/.bashrc
   sggit add ~/.vimrc
   ```

2. **Capture current state**:
   ```bash
   sggit update
   git log --oneline  # See the commit with timestamps
   ```

3. **Edit files locally**:
   ```bash
   # Edit the local copies
   vim bashrc
   vim vimrc
   
   # Commit your changes
   git add .
   git commit -m "Update shell aliases and vim plugins"
   ```

4. **Deploy changes**:
   ```bash
   sggit sync  # Updates the actual files in their original locations
   ```

5. **Regular synchronization**:
   ```bash
   # Pull in external changes
   sggit update
   
   # Push local changes
   sggit sync
   ```

## Safety Features

- **Modification date checking**: `sync` only overwrites remote files if local files are newer
- **Platform isolation**: Only processes files that match the current operating system
- **Git integration**: Full version history of all file changes
- **Non-destructive operations**: Files are copied, never moved

## Testing

Run the test suite:

```bash
cargo test
```

The tests use temporary directories to simulate file operations without affecting your actual filesystem.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

[Add your license here]

## Troubleshooting

### "File does not exist" error
Make sure the file path exists and is accessible:
```bash
ls -la /path/to/file
```

### "No such file or directory" git error
Ensure you're in a sggit repository:
```bash
ls -la .sggit/
```

### Permission denied
Check file permissions and ensure sggit has access to both source and destination paths:
```bash
chmod +r /source/file
chmod +w /destination/directory
```
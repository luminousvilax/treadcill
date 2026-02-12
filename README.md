# Todo CLI

A command-line todo list manager written in Rust. This application allows you to create, list, edit, and clear todo items directly from your terminal with persistent JSON storage.

## Features

- ✅ **Create todos** - Add new tasks with title and content
- 📝 **Interactive mode** - User-friendly interactive creation workflow
- 📋 **List todos** - View all tasks or filter by title/content
- ✏️ **Edit todos** - Modify existing tasks
- 🗑️ **Clear todos** - Remove individual tasks or clear all
- 💾 **Persistent storage** - All todos saved to JSON file
- 🎨 **User-friendly** - Built with clap for intuitive command-line interface

## Prerequisites

- **Rust** (1.70 or higher)
- **Cargo** (comes with Rust)

If you don't have Rust installed, visit [https://rustup.rs/](https://rustup.rs/) and follow the instructions.

## Installation

### Option 1: Build from Source

1. Clone or navigate to the project directory:
   ```bash
   cd /path/to/cli
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/cli`

4. (Optional) Add to PATH or create an alias:
   ```bash
   # Add to your ~/.bashrc or ~/.zshrc
   alias todo="/path/to/cli/target/release/cli"
   ```

### Option 2: Install Locally

```bash
cargo install --path .
```

This will install the binary to `~/.cargo/bin/cli` (ensure `~/.cargo/bin` is in your PATH).

## Quick Start

```bash
# Build and run in development mode
cargo run -- --help

# Or use the release binary
./target/release/cli --help
```

## Usage

### Create a New Todo

**Command-line mode:**
```bash
# Create with title and content
cli create -t "Buy groceries" -c "milk, eggs, bread"

# Create with content only (title will be empty)
cli create -c "Call dentist for appointment"
```

**Interactive mode:**
```bash
cli create -i
```

This will guide you through an interactive workflow:
1. Enter a title (press Enter to skip)
2. Enter content (required)
3. Confirm the creation (y/n)
4. Option to add more todos or quit

### List Todos

**List all todos:**
```bash
cli list
```

**Filter by title:**
```bash
cli list -t "groceries"
```

**Filter by content:**
```bash
cli list -c "dentist"
```

**Filter by both:**
```bash
cli list -t "work" -c "meeting"
```

### Edit a Todo

```bash
# Edit both title and content (uses 1-based indexing)
cli edit -i 1 -t "New Title" -c "New content"

# Edit only title
cli edit -i 2 -t "Updated Title"

# Edit only content
cli edit -i 3 -c "Updated content"
```

### Clear Todos

**Clear a specific todo:**
```bash
# Clear the first todo (1-based indexing)
cli clear -n 1
```

**Clear all todos:**
```bash
# Pass 0 or omit the number to clear everything
cli clear -n 0
# or
cli clear
```

## Command Reference

### `create` - Add a new todo

| Option | Short | Long | Description |
|--------|-------|------|-------------|
| Interactive | `-i` | N/A | Launch interactive creation mode |
| Title | `-t` | `--title` | The title of the todo (optional) |
| Content | `-c` | `--content` | The content of the todo (required in CLI mode) |

### `list` - Display todos

| Option | Short | Long | Description |
|--------|-------|------|-------------|
| Title | `-t` | `--title` | Filter by text in title |
| Content | `-c` | `--content` | Filter by text in content |

### `edit` - Modify an existing todo

| Option | Short | Long | Description |
|--------|-------|------|-------------|
| Index | `-i` | `--index` | The position of the todo to edit (1-based) |
| Title | `-t` | `--title` | New title for the todo (optional) |
| Content | `-c` | `--content` | New content for the todo (optional) |

### `clear` - Remove todos

| Option | Short | Long | Description |
|--------|-------|------|-------------|
| Number | `-n` | `--number` | Todo number to clear (1-based), 0 to clear all |

## Data Storage

Todos are stored in `data/todo.json` in JSON format. The file is automatically created on first use.

**Example storage format:**
```json
[
  {
    "title": "Buy groceries",
    "content": "milk, eggs, bread"
  },
  {
    "title": "Work tasks",
    "content": "Finish project documentation"
  }
]
```

## Development

### Project Structure

```
cli/
├── Cargo.toml           # Project dependencies and metadata
├── README               # This file
├── data/
│   └── todo.json        # Todo storage (created automatically)
└── src/
    ├── main.rs          # Application entry point
    ├── todo.rs          # Module declarations
    └── todo/
        ├── core.rs      # Core data structures and commands
        ├── create.rs    # Todo creation logic
        ├── list.rs      # Listing and filtering logic
        ├── edit.rs      # Edit functionality
        ├── clear.rs     # Clear/delete functionality
        └── storage.rs   # JSON file operations
```

### Running Tests

```bash
cargo test
```

### Dependencies

- **clap** (v4.5.41) - Command-line argument parsing
- **serde** (v1.0.219) - Serialization framework
- **serde_json** (v1.0.140) - JSON support

### Build for Development

```bash
cargo build
./target/debug/cli --help
```

### Build for Release

```bash
cargo build --release
./target/release/cli --help
```

## Examples

**Complete workflow:**

```bash
# Create your first todo
cli create -i

# Create more todos quickly
cli create -t "Exercise" -c "30 minutes cardio"
cli create -t "Study Rust" -c "Complete chapter 10"

# List all todos
cli list

# List todos about exercise
cli list -t "Exercise"

# Edit a todo
cli edit -i 1 -c "45 minutes cardio"

# Clear completed todo
cli clear -n 1

# Clear all todos
cli clear -n 0
```

## Troubleshooting

**"No such file or directory" error:**
- Ensure the `data/` directory exists (it's created automatically on first run)
- Run from the project root directory

**"Index out of bounds" when editing/clearing:**
- Check the todo list first with `cli list`
- Remember indexing starts at 1, not 0

## Version

Current version: 0.1.0

## License

This is a practice project. Feel free to use and modify as needed.
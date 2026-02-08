# shrt

A simple command runner for executing predefined command sequences from JSON files.

## Installation


**Linux/macOS:**
```bash
curl -sSf https://raw.githubusercontent.com/ozx1/shrt/main/install.sh | sh
```

**Windows (PowerShell):**
```powershell
iwr https://raw.githubusercontent.com/ozx1/shrt/main/install.ps1 | iex
```

Or download directly from [Releases](https://github.com/ozx1/shrt/releases)


## Quick Start

1. Create a `commands.json` file:
```json
{
    "dev": {
        "1": {
            "command": "npm",
            "args": ["install"]
        },
        "2": {
            "command": "npm",
            "args": ["run", "dev"]
        }
    }
}
```

2. Configure shrt:
```bash
shrt config /path/to/commands.json
```

3. Run your commands:
```bash
shrt dev
```

## Usage

- `shrt config <path>` - Set path to your commands file
- `shrt config` - Open your commands file
- `shrt <command>` - Run a command sequence
- `shrt help` - Show help

## JSON Format

Commands are defined as numbered steps:
```json
{
    "command-name": {
        "1": {
            "command": "program",
            "args": ["arg1", "arg2"]
        },
        "2": {
            "command": "another-program",
            "args": []
        }
    }
}
```

Steps execute in order. If any step fails, execution stops.

## Examples

### Development Workflow
```json
{
    "start": {
        "1": {
            "command": "npm",
            "args": ["install"]
        },
        "2": {
            "command": "npm",
            "args": ["run", "dev"]
        }
    }
}
```

### Rust Project
```json
{
    "check": {
        "1": {
            "command": "cargo",
            "args": ["fmt"]
        },
        "2": {
            "command": "cargo",
            "args": ["clippy"]
        },
        "3": {
            "command": "cargo",
            "args": ["test"]
        }
    }
}
```

## License

MIT License - see [LICENSE](LICENSE) file for details.
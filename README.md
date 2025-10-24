# Snack - A Transition Helper CLI

A CLI tool inspired by "transition snacks" to help ease transitions and overcome procrastination for neurodivergent folks.

## Inspiration

This tool combines two powerful concepts:

1. **Transition snacks** - A parenting technique I use frequently with my kids to ease transitions between activities. These small, pleasant rituals create mental breaks and acknowledge that switching contexts is genuinely hard.

2. **Dave Anderson's procrastination framework** - From his article ["I'll Explain How I Defeated Procrastination, Right After I Watch Another YouTube Video"](https://www.scarletink.com/p/ill-explain-how-i-defeated-procrastination) in the Scarlet Ink newsletter. The 5-step guide mode directly implements the practical framework from that article.

It turns out that what helps kids navigate transitions also helps neurodivergent adults (and really, all adults) overcome procrastination!

## Demo

![Demo](demo.gif)

The tool provides two modes:
- **Quick Snack Mode** (`snack`) - Instant encouragement with random ASCII art
- **Guide Mode** (`snack guide`) - Interactive 5-step process to overcome procrastination

## What is a Transition Snack?

A transition snack is a small, pleasant ritual that creates a mental break between activities, provides comfort during context switches, and acknowledges the difficulty of transitioning.

## Features

### Quick Snack Mode
Run `snack` for an instant mood boost with:
- Random ASCII art and animations
- Encouraging messages
- A moment of acknowledgment that transitions are okay

### Guide Mode
Run `snack guide` for a full 5-step procrastination-breaking process:

1. **Acknowledge** - Admit you're procrastinating (without judgment)
2. **Examine** - Understand why you have negative feelings about the task
3. **Mood Boost** - Take concrete steps to improve your mood
4. **Break Down** - Identify the tiniest first step (baby steps!)
5. **Celebrate** - Reward yourself and plan the next tiny step

## Installation

### Pre-built Binaries (Recommended)

Download the latest release for your platform from the [Releases page](https://github.com/yourusername/transition-snack/releases):

- **Linux (x86_64)**: `snack-linux-x86_64.tar.gz`
- **macOS (Intel)**: `snack-macos-x86_64.tar.gz`
- **macOS (Apple Silicon)**: `snack-macos-aarch64.tar.gz`
- **Windows (x86_64)**: `snack-windows-x86_64.zip`

All binaries are built with locked dependency versions for reproducibility and security.

#### Linux/macOS:
```bash
# Extract the archive
tar -xzf snack-*.tar.gz

# Move to a directory in your PATH
sudo mv snack /usr/local/bin/

# Make it executable (if needed)
chmod +x /usr/local/bin/snack
```

#### Windows:
```powershell
# Extract the ZIP file and move snack.exe to a directory in your PATH
# Or run it directly from the extracted location
```

### From Source

If you prefer to build from source or the pre-built binaries don't work for your platform:

```bash
# Clone or navigate to the repository
cd transition-snack

# Build the project (use --locked to match exact dependency versions)
cargo build --release --locked

# Install globally (optional)
cargo install --path . --locked

# Or run directly
cargo run
cargo run -- guide
```

## Usage

```bash
# Quick snack (random encouragement)
snack

# Full guided experience
snack guide

# Help
snack --help
```

## Design Philosophy

- **Minimal but joyful** - Quick, helpful, doesn't get in the way
- **Non-judgmental** - Acknowledges procrastination without shame
- **Baby steps** - Focus on the tiniest possible progress
- **Interactive when needed** - Quick snacks are instant; guide mode is hands-on

## Future Features

Ideas for later iterations:
- Config file support (`~/.config/snack/config.toml`)
- Custom snacks and mood boosters
- Progress logging (optional wins tracking)
- More ASCII art and animations
- Personality customization

## Contributing

This tool is meant to be joyful and helpful. If you have ideas for:
- More encouraging snacks
- Better ASCII art
- Improved prompts in guide mode
- Bug fixes

Feel free to open an issue or PR!

## License

MIT

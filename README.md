# little-bat

A minimal TUI battery status display.

## Features

- **Centered display** - Battery info displayed in the middle of the terminal
- **Two display modes**:
  - Default: Just the percentage (e.g., `85%`)
  - Graphic (`-g`): ASCII bar like `[████████░░]` with percentage below
- **Optional labels** (`-l`): Shows "Battery" header and charging state
- **Color-coded**: Green (>50%), Yellow (20-50%), Red (<20%)
- **Auto-refresh**: Updates every second
- **Exit**: Press `q` or `Esc`

## Usage

```bash
# Simple percentage only
little-bat

# With ASCII graphic
little-bat -g

# With labels
little-bat -l

# Both graphic and labels
little-bat -gl
```

## Install from source

```bash
git clone <repo>
cd little-bat
cargo install --path .
```

Then run `little-bat` from anywhere.

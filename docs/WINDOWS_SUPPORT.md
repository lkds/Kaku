# Kaku Windows Support

This branch adds Windows support to Kaku terminal.

## Status

🚧 **Work in Progress**

## Building on Windows

### Prerequisites

1. **Rust**: Install from https://rustup.rs/
2. **Visual Studio Build Tools**: Install with "Desktop development with C++" workload
3. **Git**: For cloning the repository

### Build Steps

```powershell
# Clone the repository
git clone https://github.com/YOUR_FORK/Kaku.git
cd Kaku

# Build the CLI
cargo build --release --package kaku

# Build the GUI
cargo build --release --package kaku-gui
```

### Run

```powershell
# Run the CLI
.\target\release\kaku.exe

# Run the GUI
.\target\release\kaku-gui.exe
```

## Windows-Specific Features

- **High DPI Support**: Per-monitor DPI awareness
- **Transparency**: DWM-backed window transparency
- **Clipboard**: Full clipboard integration
- **Context Menu**: Right-click menu support
- **OpenGL/WGL**: Hardware-accelerated rendering

## Known Issues

- Font rendering uses GDI fallback (DirectWrite integration pending)
- Some keyboard shortcuts may differ from macOS version

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for general contribution guidelines.

For Windows-specific issues, please file them with the `windows` label.

## CI/CD

Windows builds are automatically tested via GitHub Actions:

- **Build**: Every push to main/windows-support branches
- **Artifacts**: Compiled executables available for download
- **Cross-compile check**: Quick validation on Linux

## Architecture

```
window/src/os/windows/
├── mod.rs        # Module entry, COM initialization
├── app.rs        # Application lifecycle, message loop
├── window.rs     # Window creation, management, DPI
├── clipboard.rs  # Clipboard operations
├── keycodes.rs   # Virtual key code mapping
├── menu.rs       # Context menu, menu bar
├── gl.rs         # OpenGL/WGL context
├── bitmap.rs     # Bitmap utilities
└── connection.rs # Event handling, notifications
```

## License

MIT License - same as upstream Kaku
# Gõ Nhanh for Linux (IBus Engine)

Vietnamese Input Method Engine for Linux using IBus framework.

## Status

✅ **Phase 1 Complete**: Core IBus engine implementation
- X11 keysym → macOS keycode translation
- Integration with gonhanh_core via Rust FFI
- D-Bus interface using zbus
- Telex & VNI support

🚧 **TODO**:
- Settings UI (GTK 4)
- Config file support
- Build scripts & packaging

## Prerequisites

### Build Dependencies

```bash
sudo dnf install -y \
    rust cargo \
    dbus-devel \
    glib2-devel \
    pkg-config \
    gcc
```

### Runtime Dependencies

```bash
sudo dnf install -y ibus
```

## Building

```bash
cd platforms/linux/ibus-gonhanh
cargo build --release
```

The binary will be at: `target/release/ibus-engine-gonhanh`

## Manual Installation (for testing)

### 1. Install the engine binary

```bash
sudo install -Dm755 target/release/ibus-engine-gonhanh \
    /usr/libexec/ibus-engine-gonhanh
```

### 2. Install the component XML

```bash
sudo install -Dm644 data/gonhanh.xml \
    /usr/share/ibus/component/gonhanh.xml
```

### 3. Restart IBus

```bash
ibus restart
# Or if that doesn't work:
killall ibus-daemon
ibus-daemon -drx
```

### 4. Add GoNhanh to input sources

1. Open **Settings** → **Keyboard** → **Input Sources**
2. Click **+** (Add Input Source)
3. Select **Vietnamese**
4. Choose **Vietnamese (Gõ Nhanh)**
5. Click **Add**

### 5. Switch to GoNhanh

- Press **Super + Space** to cycle through input methods
- Or use the input source indicator in the top bar

## Testing

Open any text editor (gedit, GNOME Text Editor, Firefox, etc.) and try typing:

- **Telex**: `as` → `á`, `dd` → `đ`, `duocw` → `dươc`
- **VNI**: `a1` → `á`, `d9` → `đ`

## Debugging

Enable debug logs:

```bash
RUST_LOG=debug /usr/libexec/ibus-engine-gonhanh
```

Then in another terminal:

```bash
# Monitor IBus daemon logs
journalctl --user -u ibus -f
```

## Architecture

```
User types 'as'
  ↓
IBus Daemon
  ↓
GoNhanh Engine (D-Bus service)
  ├─ keycode.rs: X11 keysym 0x061 → macOS keycode 0
  ├─ keycode.rs: X11 keysym 0x073 → macOS keycode 1
  ↓
gonhanh_core::ime_key_ext(0) → None
gonhanh_core::ime_key_ext(1) → Send { backspace=1, chars=['á'] }
  ↓
D-Bus signals:
  1. ForwardKeyEvent(BackSpace)
  2. CommitText("á")
  ↓
Application receives: "á"
```

## Known Issues

- No settings UI yet (hardcoded to Telex mode)
- No config file support
- Debug build is large (70MB) - use `--release` for production

## Development

### Project Structure

```
platforms/linux/ibus-gonhanh/
├── src/
│   ├── main.rs        # D-Bus service setup
│   ├── engine.rs      # IBus Engine D-Bus interface
│   ├── keycode.rs     # X11 → macOS keycode translation
│   └── ffi.rs         # gonhanh_core wrapper
├── data/
│   └── gonhanh.xml    # IBus component descriptor
└── Cargo.toml
```

### Key Files

- `engine.rs`: Implements `org.freedesktop.IBus.Engine` D-Bus interface
- `keycode.rs`: Translation table from X11 keysyms to macOS virtual keycodes
- `ffi.rs`: Safe wrapper around `gonhanh_core` FFI functions

### Testing Changes

After making changes:

```bash
cargo build
sudo install target/debug/ibus-engine-gonhanh /usr/libexec/
ibus restart
```

## Next Steps

1. **Config Library** (`gonhanh-config/`)
   - TOML config at `~/.config/gonhanh/config.toml`
   - Settings: input method, tone style, auto-start

2. **Settings UI** (`gonhanh-settings/`)
   - GTK 4 + libadwaita
   - Preferences window
   - Desktop file for GNOME launcher

3. **Build Scripts**
   - `build.sh` - Build all components
   - `install.sh` - Install to system

4. **Packaging**
   - RPM spec file for Fedora
   - `rpmbuild -ba gonhanh.spec`

## License

GPL-3.0-or-later

## Author

Trieu Le <ltrungtrieu@gmail.com>

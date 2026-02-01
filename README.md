# BackupToUSB

**BackupToUSB** is a simple, cross-platform Rust application with a GTK4 GUI to back up your home directory to a USB drive or any folder.

### Features
- Copy your entire `/home/$USER` directory.
- Optional: Copy SSH keys (`~/.ssh`) for secure access.
- Optional: Delete files larger than 1 GB during backup.
- Optional: Create a `backup.log` file in the target folder.
- About dialog with logo and dynamic year display (starts from 2026).

### Usage
1. Select the **target folder** (USB or local directory).
2. Check the desired options:
   - Copy SSH keys
   - Delete large files (>1GB)
   - Add log
3. Click **Start Backup** to begin.
4. Use the **About** button to see program info with logo.

### Requirements
- Rust 2024 edition
- GTK4 (with GTK-rs bindings)
- fs_extra crate for recursive copy
- dirs crate for finding the home directory

### Installation
```bash
git clone https://github.com/Kuznix/BackupToUSB.git
cd BackupToUSB
cargo build --release

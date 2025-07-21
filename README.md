# Rust Terminal Emulator

A desktop terminal application built with Rust and egui that looks and behaves like a real terminal.

## ✨ Features

- 🦀 **Built with Rust** - Fast, safe, and reliable
- 🖥️ **Authentic Terminal Experience** - Real shell-like interface with proper prompt
- � **Directory Navigation** - Built-in `cd` command with directory tracking  
- 📚 **Command History** - Navigate through previous commands with ↑/↓ arrow keys
- 🎨 **Terminal-like Interface** - Dark theme with proper colors and monospace font
- ⌨️ **Real Keyboard Shortcuts** - Ctrl+C, Ctrl+D, Ctrl+L support
- 🔄 **Blinking Cursor** - Authentic terminal cursor that blinks
- 🚀 **Real Command Execution** - Execute actual system commands

## 🖼️ What Makes This Special

This isn't just a command executor - it's designed to **look and feel exactly like a real terminal**:

- **Proper Shell Prompt**: `user@hostname:~/path $` 
- **Directory Tracking**: Shows current directory and supports `cd`
- **Authentic Colors**: Green prompts, colored output, error highlighting
- **Real Terminal Behavior**: Cursor positioning, proper input handling
- **Command History**: Just like bash - use ↑/↓ to navigate history

## 🎮 Built-in Commands

- `cd [directory]` - Change directory (supports relative and absolute paths)
- `pwd` - Print current working directory  
- `clear` - Clear the terminal screen
- `history` - Show command history
- `exit` - Exit the application

## 🎯 Keyboard Shortcuts

- **Enter** - Execute command
- **↑/↓ arrows** - Navigate command history  
- **←/→ arrows** - Move cursor in input line
- **Home/End** - Jump to beginning/end of line
- **Ctrl+C** - Interrupt/cancel current input
- **Ctrl+L** - Clear screen  
- **Ctrl+D** - Exit application
- **Tab** - Insert space (tab completion could be added)

## 🚀 Usage

### Running the Application

```bash
cargo run
```

### Building for Release

```bash
./build.sh
# or manually:
cargo build --release
```

### Using the Terminal

1. **Natural terminal experience** - just type commands like in any terminal
2. **Navigation**: `cd ..`, `cd /home`, `cd Documents`  
3. **File operations**: `ls -la`, `cat filename.txt`, `mkdir newdir`
4. **System commands**: `ps aux`, `whoami`, `date`, `uname -a`
5. **History**: Use ↑/↓ to recall previous commands

## 📝 Example Session

```
user@hostname:~ $ ls
Documents  Downloads  Desktop  Pictures
user@hostname:~ $ cd Documents  
user@hostname:~/Documents $ pwd
/home/user/Documents
user@hostname:~/Documents $ ls -la
total 48
drwxr-xr-x  3 user user  4096 Jan 15 10:30 .
drwxr-xr-x 15 user user  4096 Jan 15 09:15 ..
-rw-r--r--  1 user user  1024 Jan 15 10:30 README.md
user@hostname:~/Documents $ history
 1: ls
 2: cd Documents  
 3: pwd
 4: ls -la
 5: history
user@hostname:~/Documents $ clear
user@hostname:~/Documents $ 
```

## 🔧 Technical Details

- **GUI Framework**: egui/eframe for native cross-platform GUI
- **Command Execution**: `std::process::Command` for system command execution  
- **Directory Management**: Built-in `cd` and `pwd` with proper path resolution
- **History**: In-memory command history with navigation
- **Cursor**: Blinking terminal cursor with proper positioning
- **Colors**: Authentic terminal color scheme (green prompts, red errors)

## 🎨 Visual Design

- **Dark terminal background** (#0C0C14) 
- **Monospace font** for authentic terminal feel
- **Proper spacing and margins**
- **Color-coded output**:
  - 🟢 Green: Prompts and user@hostname
  - 🟡 Yellow: User input/commands  
  - 🔴 Red: Error messages
  - ⚪ White: Normal command output

## 🔮 Future Enhancements

- Tab completion for files and commands
- Real PTY integration for interactive programs  
- ANSI color code support
- Terminal resizing
- Copy/paste functionality
- Split terminal panes

## 📦 Dependencies

- `eframe` - Cross-platform GUI framework
- `egui` - Immediate mode GUI library

## 📄 License

This project is open source and available under the MIT License.

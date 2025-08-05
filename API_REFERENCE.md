# API Reference

This document provides a complete API reference for the AI Terminal codebase, documenting all structs, functions, and their usage patterns.

## Table of Contents
1. [Core Structures](#core-structures)
2. [Main Application](#main-application)
3. [Public Methods](#public-methods)
4. [Internal Methods](#internal-methods)
5. [Event Handling](#event-handling)
6. [Constants and Types](#constants-and-types)

## Core Structures

### `TerminalLine`

```rust
#[derive(Clone)]
struct TerminalLine {
    text: String,
    is_input: bool,
    is_prompt: bool,
}
```

Represents a single line in the terminal output buffer.

**Fields:**
- `text: String` - The text content of the line
- `is_input: bool` - `true` if this line represents user input, `false` for system output
- `is_prompt: bool` - `true` if this line is a prompt header, `false` for regular content

**Traits:**
- `Clone` - Allows efficient copying of terminal lines

**Usage:**
```rust
let line = TerminalLine {
    text: "Hello, World!".to_string(),
    is_input: false,
    is_prompt: false,
};
```

### `TerminalApp`

```rust
struct TerminalApp {
    lines: VecDeque<TerminalLine>,
    input_buffer: String,
    cursor_pos: usize,
    show_cursor: bool,
    last_cursor_blink: Instant,
    command_history: Vec<String>,
    history_index: isize,
    current_dir: String,
    username: String,
    hostname: String,
    autocomplete_suggestions: Vec<String>,
    autocomplete_index: isize,
    show_autocomplete: bool,
    common_commands: Vec<String>,
    command_flags: HashMap<String, Vec<String>>,
}
```

Main application state containing all terminal data and UI state.

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `lines` | `VecDeque<TerminalLine>` | Terminal output buffer with FIFO semantics |
| `input_buffer` | `String` | Current user input being typed |
| `cursor_pos` | `usize` | Current cursor position in input buffer |
| `show_cursor` | `bool` | Cursor visibility state for blinking animation |
| `last_cursor_blink` | `Instant` | Timestamp of last cursor blink toggle |
| `command_history` | `Vec<String>` | Previously executed commands |
| `history_index` | `isize` | Current position in command history (-1 = not browsing) |
| `current_dir` | `String` | Current working directory path |
| `username` | `String` | System username |
| `hostname` | `String` | System hostname |
| `autocomplete_suggestions` | `Vec<String>` | Current autocomplete suggestions |
| `autocomplete_index` | `isize` | Selected autocomplete suggestion (-1 = none) |
| `show_autocomplete` | `bool` | Whether to display autocomplete suggestions |
| `common_commands` | `Vec<String>` | Database of known commands for completion |
| `command_flags` | `HashMap<String, Vec<String>>` | Command-specific flags for completion |

## Main Application

### `main() -> Result<(), eframe::Error>`

Application entry point that configures and launches the terminal emulator.

**Returns:** `Result<(), eframe::Error>` - Success or framework error

**Configuration:**
- Window size: 1000x700 pixels
- Theme: Dark mode with deep blue background (RGB: 12, 12, 20)
- Resizable: Yes
- Title: "Terminal"

**Example:**
```rust
fn main() -> Result<(), eframe::Error> {
    // Window configuration
    let options = eframe::NativeOptions { /* ... */ };
    
    // Launch application
    eframe::run_native("Terminal", options, Box::new(|cc| {
        // Theme setup
        let mut visuals = egui::Visuals::dark();
        // ...
        Ok(Box::new(TerminalApp::new()))
    }))
}
```

## Public Methods

### `TerminalApp::new() -> Self`

Creates and initializes a new terminal application instance.

**Returns:** `TerminalApp` - Fully initialized terminal application

**Initialization Process:**
1. Detects current working directory with fallback to "/"
2. Extracts username from USER environment variable
3. Determines hostname from HOSTNAME or `hostname` command
4. Populates command database with common commands
5. Sets up autocomplete flags for known commands
6. Displays welcome message and initial prompt

**Environment Variables Used:**
- `USER` - System username (fallback: "user")
- `HOSTNAME` - System hostname (fallback: execute `hostname` command)
- `HOME` - User home directory

**Example:**
```rust
let app = TerminalApp::new();
// App is ready to use with initial state
```

### `add_line(&mut self, text: &str, is_input: bool, is_prompt: bool)`

Adds a new line to the terminal output buffer with automatic memory management.

**Parameters:**
- `text: &str` - The text content to add
- `is_input: bool` - Whether this represents user input
- `is_prompt: bool` - Whether this is a prompt line

**Memory Management:**
- Automatically removes oldest lines when buffer exceeds 500 lines
- Uses `VecDeque` for efficient front/back operations
- Prevents unbounded memory growth

**Example:**
```rust
// Add system output
app.add_line("File not found", false, false);

// Add user input
app.add_line("ls -la", true, false);

// Add prompt
app.add_line("user@desktop > ", false, true);
```

### `show_prompt(&mut self)`

Displays a PowerShell-inspired prompt with system information and Git integration.

**Features:**
- Converts full paths to relative display (~/... format)
- Shows only current directory name for clarity
- Dynamically includes Git branch information when in repository
- Uses Windows-style backslashes for PowerShell aesthetic
- Colorful emoji-enhanced header

**Format:**
```
🏠 username@Desktop 📂 ~\path\directory 🐧 3.9.1 ⚡ branch_name
> 
```

**Example:**
```rust
app.show_prompt();
// Displays formatted prompt with current context
```

### `execute_command(&mut self, command: &str)`

Executes a command, handling both built-in and external commands.

**Parameters:**
- `command: &str` - The command string to execute

**Built-in Commands:**
- `help` - Shows help information
- `clear` - Clears the terminal screen
- `exit` - Terminates the application
- `cd [path]` - Changes current directory
- `pwd` - Prints working directory
- `history` - Shows command history

**External Command Handling:**
- Spawns system processes using `std::process::Command`
- Captures stdout, stderr, and exit codes
- Displays output with appropriate formatting
- Handles command not found errors

**History Management:**
- Automatically adds commands to history
- Prevents duplicate consecutive entries
- Resets history navigation state

**Example:**
```rust
app.execute_command("ls -la");
app.execute_command("cd /home");
app.execute_command("git status");
```

## Internal Methods

### `get_git_branch(&self) -> String`

Detects Git repository and returns formatted branch information.

**Returns:** `String` - Formatted branch info or empty string

**Detection Logic:**
1. Executes `git rev-parse --abbrev-ref HEAD` in current directory
2. Validates command success and output
3. Filters out empty strings and "HEAD" (detached state)
4. Formats with lightning emoji: "⚡ branch_name"

**Error Handling:**
- Returns empty string for non-Git directories
- Gracefully handles Git command failures
- No error messages for normal non-Git usage

**Example:**
```rust
let git_info = app.get_git_branch();
// Returns "⚡ main" if in Git repo on main branch
// Returns "" if not in Git repository
```

### `update_autocomplete(&mut self)`

Updates autocomplete suggestions based on current input context.

**Algorithm:**
1. **Input Validation** - Skip if input is empty
2. **Context Parsing** - Determine current word being typed
3. **Completion Type** - Commands vs flags based on position
4. **Filtering** - Match prefixes only
5. **Performance Limit** - Maximum 5 suggestions

**Context Awareness:**
- **First word**: Complete from common commands
- **Subsequent words starting with '-'**: Complete command-specific flags
- **Other words**: No completion (could be extended for file completion)

**Performance Optimizations:**
- Early termination when suggestion limit reached
- Minimum input length requirement
- Efficient string prefix matching

**Example:**
```rust
// Input: "gi"
app.update_autocomplete();
// Suggestions: ["git"]

// Input: "git st"  
app.update_autocomplete();
// Suggestions: ["status"]

// Input: "ls -"
app.update_autocomplete();  
// Suggestions: ["-l", "-a", "-la", "-lh"]
```

### `apply_autocomplete(&mut self) -> bool`

Applies the currently selected autocomplete suggestion to input buffer.

**Returns:** `bool` - `true` if suggestion was applied, `false` if none available

**Behavior:**
- Cycles through suggestions on repeated calls
- Replaces current word with selected suggestion
- Adds space after commands and flags for continued typing
- Updates cursor position appropriately

**Example:**
```rust
// Input: "gi" with suggestion "git"
let applied = app.apply_autocomplete();
// Input becomes: "git " (with trailing space)
// Returns: true
```

### `handle_key(&mut self, key: egui::Key, modifiers: egui::Modifiers)`

Processes keyboard input and updates application state accordingly.

**Parameters:**
- `key: egui::Key` - The pressed key
- `modifiers: egui::Modifiers` - Modifier keys (Ctrl, Alt, Shift)

**Key Handling:**

| Key | Action |
|-----|--------|
| `Enter` | Execute current command |
| `Backspace` | Delete character before cursor |
| `Delete` | Delete character at cursor |
| `ArrowLeft/Right` | Move cursor |
| `ArrowUp/Down` | Navigate command history |
| `Home/End` | Move cursor to start/end |
| `Tab` | Apply autocomplete or insert space |
| `Escape` | Hide autocomplete suggestions |

**Modifier Combinations:**

| Combination | Action |
|-------------|--------|
| `Ctrl+C` | Interrupt current input |
| `Ctrl+D` | Exit application |
| `Ctrl+L` | Clear screen |

**State Management:**
- Updates autocomplete after input changes
- Manages cursor position bounds
- Clears autocomplete during history navigation
- Handles edge cases gracefully

## Event Handling

### `update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)`

Main update loop implementing the `eframe::App` trait.

**Parameters:**
- `ctx: &egui::Context` - GUI context for rendering and input
- `_frame: &mut eframe::Frame` - Application frame (unused)

**Update Phases:**

1. **Cursor Blinking**
   ```rust
   if self.last_cursor_blink.elapsed() > Duration::from_millis(500) {
       self.show_cursor = !self.show_cursor;
       self.last_cursor_blink = Instant::now();
       ctx.request_repaint_after(Duration::from_millis(500));
   }
   ```

2. **Event Processing**
   ```rust
   ctx.input(|i| {
       for event in &i.events {
           match event {
               egui::Event::Key { key, pressed: true, modifiers, .. } => {
                   self.handle_key(*key, *modifiers);
               }
               egui::Event::Text(text) => {
                   // Character insertion
               }
               _ => {}
           }
       }
   });
   ```

3. **UI Rendering**
   ```rust
   egui::CentralPanel::default()
       .frame(egui::Frame::none().fill(egui::Color32::from_rgb(12, 12, 20)))
       .show(ctx, |ui| {
           // Terminal content rendering
       });
   ```

**Performance Optimizations:**
- Selective repainting for cursor blinking
- Efficient event batching
- Minimal UI state updates

### Text Input Processing

Character insertion logic with control character filtering:

```rust
egui::Event::Text(text) => {
    for ch in text.chars() {
        if ch.is_control() || ch == '\n' || ch == '\r' {
            continue;  // Skip control characters
        }
        self.input_buffer.insert(self.cursor_pos, ch);
        self.cursor_pos += 1;
    }
    self.update_autocomplete();
}
```

**Features:**
- Filters out control characters
- Inserts characters at cursor position
- Updates cursor position
- Triggers autocomplete update

## Constants and Types

### Color Constants

```rust
// Background colors
const BACKGROUND_COLOR: egui::Color32 = egui::Color32::from_rgb(12, 12, 20);
const HEADER_BACKGROUND: egui::Color32 = egui::Color32::from_rgb(30, 30, 40);

// Text colors
const ERROR_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 100, 100);
const INPUT_COLOR: egui::Color32 = egui::Color32::from_rgb(255, 255, 100);
const PROMPT_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 255, 150);
const NORMAL_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 220, 220);
```

### Performance Constants

```rust
const MAX_TERMINAL_LINES: usize = 500;  // Buffer size limit
const MAX_AUTOCOMPLETE_SUGGESTIONS: usize = 5;  // Suggestion limit
const CURSOR_BLINK_INTERVAL: Duration = Duration::from_millis(500);  // Blink timing
```

### Font Configuration

```rust
const MAIN_FONT_SIZE: f32 = 18.0;    // Main terminal text
const HEADER_FONT_SIZE: f32 = 16.0;  // PowerShell headers
const MONOSPACE_FONT: egui::FontId = egui::FontId::monospace(18.0);
```

## Usage Patterns

### Basic Terminal Operations

```rust
// Create and initialize terminal
let mut app = TerminalApp::new();

// Add content
app.add_line("System started", false, false);
app.show_prompt();

// Execute commands
app.execute_command("ls -la");
app.execute_command("cd /home");

// Handle user input
app.handle_key(egui::Key::Tab, egui::Modifiers::NONE);  // Autocomplete
app.handle_key(egui::Key::Enter, egui::Modifiers::NONE);  // Execute
```

### Extending Commands

```rust
// In execute_command() match statement
"custom_command" => {
    // Custom logic here
    self.add_line("Custom output", false, false);
    self.show_prompt();
    return;
}
```

### Adding Autocomplete Support

```rust
// In new() initialization
command_flags.insert("custom_cmd".to_string(), vec![
    "--flag1".to_string(),
    "--flag2".to_string(),
]);

common_commands.push("custom_cmd".to_string());
```

This API reference provides complete documentation for integrating with and extending the AI Terminal codebase.

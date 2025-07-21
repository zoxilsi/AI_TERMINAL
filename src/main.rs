use eframe::egui;
use std::collections::VecDeque;
use std::process::Command;
use std::time::{Duration, Instant};
use std::env;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Terminal")
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Terminal",
        options,
        Box::new(|cc| {
            // Set up authentic terminal theme
            let mut visuals = egui::Visuals::dark();
            visuals.window_fill = egui::Color32::from_rgb(12, 12, 20);
            visuals.panel_fill = egui::Color32::from_rgb(12, 12, 20);
            visuals.extreme_bg_color = egui::Color32::from_rgb(12, 12, 20);
            cc.egui_ctx.set_visuals(visuals);
            
            Ok(Box::new(TerminalApp::new()))
        }),
    )
}

#[derive(Clone)]
struct TerminalLine {
    text: String,
    is_input: bool,
    is_prompt: bool,
}

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
}

impl TerminalApp {
    fn new() -> Self {
        let current_dir = env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("/"))
            .to_string_lossy()
            .to_string();
        
        let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
        let hostname = env::var("HOSTNAME").unwrap_or_else(|_| {
            // Try to get hostname from system
            Command::new("hostname")
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
                .unwrap_or_else(|_| "localhost".to_string())
        });

        let mut app = Self {
            lines: VecDeque::new(),
            input_buffer: String::new(),
            cursor_pos: 0,
            show_cursor: true,
            last_cursor_blink: Instant::now(),
            command_history: Vec::new(),
            history_index: -1,
            current_dir,
            username,
            hostname,
        };

        // Add welcome message
        app.add_line("Welcome to Rust Terminal Emulator", false, false);
        app.add_line("This terminal emulates a real bash shell with proper prompt and directory tracking", false, false);
        app.add_line("", false, false);
        
        // Show initial prompt
        app.show_prompt();
        
        app
    }

    fn add_line(&mut self, text: &str, is_input: bool, is_prompt: bool) {
        self.lines.push_back(TerminalLine {
            text: text.to_string(),
            is_input,
            is_prompt,
        });
        
        // Keep buffer manageable
        while self.lines.len() > 1000 {
            self.lines.pop_front();
        }
    }

    fn show_prompt(&mut self) {
        let home = env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        let display_dir = if self.current_dir.starts_with(&home) {
            self.current_dir.replace(&home, "~")
        } else {
            self.current_dir.clone()
        };
        
        let prompt = format!("{}@{}:{} $ ", self.username, self.hostname, display_dir);
        self.add_line(&prompt, false, true);
    }

    fn execute_command(&mut self, command: &str) {
        if command.trim().is_empty() {
            self.show_prompt();
            return;
        }

        // Add to history
        if !command.trim().is_empty() && (self.command_history.is_empty() || self.command_history.last() != Some(&command.to_string())) {
            self.command_history.push(command.to_string());
        }
        self.history_index = -1;

        // Show the command being executed
        self.add_line(command, true, false);

        let parts: Vec<String> = command.trim().split_whitespace().map(|s| s.to_string()).collect();
        if parts.is_empty() {
            self.show_prompt();
            return;
        }

        let cmd_name = parts[0].clone();
        let args: Vec<String> = parts[1..].to_vec();

        // Handle built-in commands
        match cmd_name.as_str() {
            "clear" => {
                self.lines.clear();
                self.show_prompt();
                return;
            }
            "exit" => {
                std::process::exit(0);
            }
            "cd" => {
                let target_dir = if args.is_empty() {
                    env::var("HOME").unwrap_or_else(|_| "/".to_string())
                } else {
                    args[0].clone()
                };
                
                let new_path = if target_dir.starts_with('/') {
                    std::path::PathBuf::from(&target_dir)
                } else {
                    std::path::PathBuf::from(&self.current_dir).join(&target_dir)
                };
                
                match new_path.canonicalize() {
                    Ok(canonical_path) => {
                        if canonical_path.is_dir() {
                            self.current_dir = canonical_path.to_string_lossy().to_string();
                            let _ = env::set_current_dir(&canonical_path);
                        } else {
                            self.add_line(&format!("cd: {}: Not a directory", target_dir), false, false);
                        }
                    }
                    Err(_) => {
                        self.add_line(&format!("cd: {}: No such file or directory", target_dir), false, false);
                    }
                }
                self.show_prompt();
                return;
            }
            "pwd" => {
                let pwd = self.current_dir.clone();
                self.add_line(&pwd, false, false);
                self.show_prompt();
                return;
            }
            "history" => {
                let history = self.command_history.clone();
                for (i, cmd) in history.iter().enumerate() {
                    let history_line = format!(" {}: {}", i + 1, cmd);
                    self.add_line(&history_line, false, false);
                }
                self.show_prompt();
                return;
            }
            _ => {}
        }

        // Execute external command synchronously for now
        let result = Command::new(&cmd_name)
            .args(&args)
            .current_dir(&self.current_dir)
            .output();

        match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                // Add stdout
                for line in stdout.lines() {
                    self.add_line(line, false, false);
                }
                
                // Add stderr
                for line in stderr.lines() {
                    if !line.is_empty() {
                        self.add_line(&format!("ERROR: {}", line), false, false);
                    }
                }
                
                // Add exit status if non-zero
                if !output.status.success() {
                    if let Some(code) = output.status.code() {
                        self.add_line(&format!("Command '{}' exited with code {}", cmd_name, code), false, false);
                    }
                }
            }
            Err(e) => {
                self.add_line(&format!("Failed to execute '{}': {}", cmd_name, e), false, false);
            }
        }

        self.show_prompt();
    }

    fn handle_key(&mut self, key: egui::Key, modifiers: egui::Modifiers) {
        match key {
            egui::Key::Enter => {
                let command = self.input_buffer.clone();
                self.input_buffer.clear();
                self.cursor_pos = 0;
                self.execute_command(&command);
            }
            egui::Key::Backspace => {
                if self.cursor_pos > 0 {
                    self.input_buffer.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                }
            }
            egui::Key::Delete => {
                if self.cursor_pos < self.input_buffer.len() {
                    self.input_buffer.remove(self.cursor_pos);
                }
            }
            egui::Key::ArrowLeft => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            egui::Key::ArrowRight => {
                if self.cursor_pos < self.input_buffer.len() {
                    self.cursor_pos += 1;
                }
            }
            egui::Key::ArrowUp => {
                if !self.command_history.is_empty() {
                    if self.history_index < 0 {
                        self.history_index = self.command_history.len() as isize - 1;
                    } else if self.history_index > 0 {
                        self.history_index -= 1;
                    }
                    if self.history_index >= 0 {
                        self.input_buffer = self.command_history[self.history_index as usize].clone();
                        self.cursor_pos = self.input_buffer.len();
                    }
                }
            }
            egui::Key::ArrowDown => {
                if !self.command_history.is_empty() && self.history_index >= 0 {
                    self.history_index += 1;
                    if self.history_index >= self.command_history.len() as isize {
                        self.history_index = -1;
                        self.input_buffer.clear();
                        self.cursor_pos = 0;
                    } else {
                        self.input_buffer = self.command_history[self.history_index as usize].clone();
                        self.cursor_pos = self.input_buffer.len();
                    }
                }
            }
            egui::Key::Home => {
                self.cursor_pos = 0;
            }
            egui::Key::End => {
                self.cursor_pos = self.input_buffer.len();
            }
            egui::Key::Tab => {
                // Simple tab completion could be added here
                self.input_buffer.push(' ');
                self.cursor_pos += 1;
            }
            _ => {
                if modifiers.ctrl {
                    match key {
                        egui::Key::C => {
                            // Ctrl+C - interrupt current command
                            self.add_line("^C", false, false);
                            self.input_buffer.clear();
                            self.cursor_pos = 0;
                            self.show_prompt();
                        }
                        egui::Key::D => {
                            // Ctrl+D - EOF/exit
                            std::process::exit(0);
                        }
                        egui::Key::L => {
                            // Ctrl+L - clear screen
                            self.lines.clear();
                            self.show_prompt();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

impl eframe::App for TerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle cursor blinking
        if self.last_cursor_blink.elapsed() > Duration::from_millis(500) {
            self.show_cursor = !self.show_cursor;
            self.last_cursor_blink = Instant::now();
        }

        // Request continuous repaints for cursor blinking
        ctx.request_repaint_after(Duration::from_millis(100));

        // Handle keyboard input
        ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Key { key, pressed: true, modifiers, .. } => {
                        self.handle_key(*key, *modifiers);
                    }
                    egui::Event::Text(text) => {
                        for ch in text.chars() {
                            if ch.is_control() || ch == '\n' || ch == '\r' {
                                continue;
                            }
                            self.input_buffer.insert(self.cursor_pos, ch);
                            self.cursor_pos += 1;
                        }
                    }
                    _ => {}
                }
            }
        });

        // Main terminal panel - fullscreen
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(12, 12, 20)))
            .show(ctx, |ui| {
                // Terminal content with proper margins
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(12, 12, 20))
                    .inner_margin(egui::Margin::same(12.0))
                    .show(ui, |ui| {
                        // Scrollable terminal area
                        egui::ScrollArea::vertical()
                            .stick_to_bottom(true)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                                    // Display all terminal lines
                                    for line in &self.lines {
                                        let color = if line.text.starts_with("ERROR:") {
                                            egui::Color32::from_rgb(255, 100, 100) // Red for errors
                                        } else if line.is_prompt {
                                            egui::Color32::from_rgb(100, 255, 100) // Green for prompts
                                        } else if line.is_input {
                                            egui::Color32::from_rgb(255, 255, 100) // Yellow for input
                                        } else {
                                            egui::Color32::from_rgb(220, 220, 220) // Normal text
                                        };
                                        
                                        ui.label(
                                            egui::RichText::new(&line.text)
                                                .font(egui::FontId::monospace(14.0))
                                                .color(color)
                                        );
                                    }

                                    // Current input line with cursor
                                    if let Some(last_line) = self.lines.back() {
                                        if last_line.is_prompt {
                                            ui.horizontal(|ui| {
                                                // Show the input with cursor
                                                let mut display_input = self.input_buffer.clone();
                                                
                                                // Add blinking cursor
                                                if self.show_cursor {
                                                    if self.cursor_pos >= display_input.len() {
                                                        display_input.push('█');
                                                    } else {
                                                        display_input.insert(self.cursor_pos, '█');
                                                    }
                                                }

                                                ui.label(
                                                    egui::RichText::new(&display_input)
                                                        .font(egui::FontId::monospace(14.0))
                                                        .color(egui::Color32::from_rgb(220, 220, 220))
                                                );
                                            });
                                        }
                                    }
                                });
                            });

                        // Status bar
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.small(format!("Terminal - {} | Ctrl+C: interrupt | Ctrl+L: clear | Ctrl+D: exit", self.current_dir));
                        });
                    });
            });
    }
}

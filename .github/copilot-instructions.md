<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

This is a Rust desktop terminal application project. The application should:

1. Use egui for the cross-platform GUI framework
2. Use portable-pty for terminal process spawning and management
3. Provide a functional terminal interface that works like a normal system terminal
4. Support basic terminal features like:
   - Command execution
   - Text input/output
   - Command history
   - Copy/paste functionality
   - Scrollback buffer
5. Have a clean, modern UI that resembles a typical terminal emulator

Key dependencies to use:
- egui and eframe for GUI
- portable-pty for terminal backend
- crossterm for terminal handling
- tokio for async operations

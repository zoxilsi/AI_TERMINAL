# Security Policy

## Supported Versions

We actively support the following versions of AI Terminal:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability in AI Terminal, please report it responsibly.

### How to Report

**DO NOT** create a public GitHub issue for security vulnerabilities.

Instead, please:

1. **Email**: Send details to [security@your-domain.com] (replace with your email)
2. **Subject**: Include "AI Terminal Security Vulnerability" in the subject line
3. **Include**:
   - Description of the vulnerability
   - Steps to reproduce the issue
   - Potential impact
   - Any suggested fixes (if you have them)

### What to Expect

- **Acknowledgment**: We'll acknowledge receipt within 48 hours
- **Initial Assessment**: We'll provide an initial assessment within 5 business days
- **Updates**: We'll keep you informed of our progress
- **Resolution**: We aim to resolve critical vulnerabilities within 30 days

### Security Considerations

AI Terminal is a desktop application that:

- **Executes system commands**: Uses `std::process::Command` to run shell commands
- **File system access**: Can navigate and access files based on user permissions
- **No network communication**: Currently doesn't make network requests (except Git commands)
- **Local data only**: All data stays on the user's machine

### Known Security Limitations

1. **Command Execution**: The terminal executes any command the user types - this is by design but means users should be careful with untrusted input
2. **File Access**: The application can access any files the user has permissions for
3. **Git Integration**: Git commands are executed which could potentially access remote repositories

### Best Practices for Users

- **Verify commands**: Always verify commands before executing them
- **Trust sources**: Only run commands from trusted sources
- **Keep updated**: Use the latest version of AI Terminal
- **System security**: Keep your operating system and security software updated

### Security-Related Dependencies

We monitor our dependencies for security vulnerabilities:

- **egui/eframe**: GUI framework dependencies
- **std library**: Standard Rust library components

### Disclosure Policy

- We'll work with you to understand and resolve the issue
- We'll provide credit for responsible disclosure (if you want it)
- We'll coordinate public disclosure after the vulnerability is fixed
- We may provide a security advisory if the vulnerability is significant

## Contact

For security-related questions or concerns:
- Email: [your-security-email@domain.com]
- Create a private issue if email isn't available

Thank you for helping keep AI Terminal secure! 🔒

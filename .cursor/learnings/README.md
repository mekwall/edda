# Project Learnings

This directory contains documented learnings, insights, and lessons learned during the development of the Lumir project.

## Purpose

The goal is to prevent knowledge loss and improve future development by documenting:

- Complex problems and their solutions
- Platform-specific behaviors and workarounds
- Performance insights and optimizations
- Architecture decisions and their rationale
- Third-party library integration issues

## File Organization

- **`PROJECT_LEARNINGS.md`** - General project insights and cross-cutting concerns
- **`RUST_LEARNINGS.md`** - Rust-specific patterns, issues, and solutions
- **`FLUTTER_LEARNINGS.md`** - Flutter/Dart-specific learnings
- **`FFI_LEARNINGS.md`** - FFI (Foreign Function Interface) between Rust and Dart
- **`WINDOWS_LEARNINGS.md`** - Windows-specific behaviors and issues
- **`MACOS_LEARNINGS.md`** - macOS-specific behaviors and issues
- **`LINUX_LEARNINGS.md`** - Linux-specific behaviors and issues

## How to Use

1. **Document as you go** - Don't wait to document learnings
2. **Be specific** - Include exact error messages, code snippets, and version numbers
3. **Include context** - What environment and conditions led to the issue
4. **Link resources** - Reference Stack Overflow, GitHub issues, documentation
5. **Tag appropriately** - Use tags for easy searching

## Template

Use this template for new learning entries:

```markdown
## [Date] - [Brief Description]

**Context**: What were you working on?

**Problem**: What issue or challenge did you encounter?

**Investigation**: What did you try? What research did you do?

**Solution**: What finally worked? Include code examples if relevant.

**Why It Happened**: Root cause analysis if applicable.

**Prevention**: How to avoid this in the future?

**Related**: Links to issues, PRs, or related learnings.
```

## Maintenance

- Review learnings quarterly and remove outdated information
- Update references when learnings become outdated
- Cross-reference related learnings
- Validate documented solutions periodically

## Integration

- Reference learnings in TODO files when creating new tasks
- Include learning references in commit messages when addressing documented issues
- Link to learnings in code comments for complex solutions

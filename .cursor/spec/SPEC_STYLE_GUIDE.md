# Edda - Specification Style Guide

This document defines the comprehensive style guide and documentation standards for all Edda specification documents. This guide ensures consistency, clarity, and maintainability across all specification documents.

---

## 📋 Document Structure

### Document Header

Each specification should begin with a clear, descriptive title and a brief overview:

```markdown
# Specification Name

Brief description of what this specification covers and its purpose within the Edda architecture.
```

### Section Organization

1. **Overview/Introduction** - Purpose and scope
2. **Architecture Context** - How this fits into the overall architecture
3. **Core Concepts** - Key definitions and concepts
4. **Implementation Details** - Technical specifications and code examples
5. **Integration Points** - How this component interacts with others
6. **Configuration** - Configuration options and patterns
7. **Error Handling** - Error scenarios and handling strategies
8. **Performance Considerations** - Performance implications and optimizations
9. **Security Considerations** - Security implications and best practices
10. **Testing Strategy** - Testing approaches and requirements

---

## 📝 Writing Standards

### Language and Tone

- **Clear and Concise**: Use simple, direct language
- **Technical Accuracy**: Ensure all technical details are precise
- **Consistent Terminology**: Use defined terms consistently
- **Active Voice**: Prefer active voice over passive voice
- **Present Tense**: Use present tense for current state and future plans

### Code Examples

#### Rust Code Blocks

```rust
/// Example function with documentation
pub fn example_function(param: String) -> Result<(), Error> {
    // Implementation with clear comments
    Ok(())
}
```

#### Configuration Examples

```toml
# Configuration example with comments
[section]
key = "value"  # Description of what this does
```

#### Command Examples

```bash
# Command with explanation
edda task create "Example task" --priority high
```

### Cross-References

Use consistent cross-reference patterns:

- **Internal References**: `[SPEC_ARCHITECTURE.md](./SPEC_ARCHITECTURE.md)`
- **Section References**: `[Error Handling](#error-handling)`
- **External References**: `[Rust Book](https://doc.rust-lang.org/book/)`

---

## 🏷️ Naming Conventions

### Document Names

- **Format**: `SPEC_<COMPONENT>_<ASPECT>.md`
- **Examples**:
  - `SPEC_ARCHITECTURE.md`
  - `SPEC_CLI_DESIGN.md`
  - `SPEC_TASK_MANAGEMENT.md`

### Section Headers

- **Level 1**: `# Main Section`
- **Level 2**: `## Subsection`
- **Level 3**: `### Detail Section`
- **Level 4**: `#### Implementation Detail`

### Code Elements

#### Rust Types and Functions

```rust
// Structs: PascalCase
pub struct TaskManager {
    // Fields: snake_case
    task_store: Arc<TaskStore>,
}

// Functions: snake_case
pub fn create_task(&self, task: Task) -> Result<TaskId, Error> {
    // Implementation
}

// Enums: PascalCase
pub enum TaskStatus {
    // Variants: PascalCase
    Pending,
    Waiting,
    Completed,
    Deleted,
}
```

#### Configuration Keys

```toml
# Use snake_case for configuration keys
[task_management]
max_concurrent_tasks = 10
default_priority = "medium"
```

#### Command Names

```bash
# Use kebab-case for command names
edda task-create "Task name"
edda sync-provider list
```

---

## 🔗 Cross-Reference Patterns

### Internal Document References

```markdown
> **Note**: For detailed information about X, see [SPEC_Y.md](./SPEC_Y.md).
```

### Section References

```markdown
See the [Error Handling](#error-handling) section for details.
```

### External References

```markdown
For more information, see the [Rust Documentation](https://doc.rust-lang.org/).
```

---

## 📊 Code Documentation Standards

### Rust Documentation

#### Function Documentation

````rust
/// Brief description of what the function does
///
/// # Arguments
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
/// Description of return value
///
/// # Errors
/// Description of error conditions
///
/// # Examples
/// ```rust
/// let result = function_name("example")?;
/// ```
pub fn function_name(param1: &str, param2: u32) -> Result<String, Error> {
    // Implementation
}
````

#### Struct Documentation

```rust
/// Brief description of the struct
///
/// This struct is used for...
#[derive(Debug, Clone)]
pub struct ExampleStruct {
    /// Description of field1
    pub field1: String,
    /// Description of field2
    pub field2: u32,
}
```

#### Enum Documentation

```rust
/// Brief description of the enum
#[derive(Debug, Clone)]
pub enum ExampleEnum {
    /// Description of variant1
    Variant1,
    /// Description of variant2 with data
    Variant2(String),
}
```

### Error Documentation

```rust
/// Brief description of the error
#[derive(Debug, thiserror::Error)]
pub enum ExampleError {
    /// Description of this error variant
    #[error("Human-readable error message: {0}")]
    Variant1(String),

    /// Description of another error variant
    #[error("Another error: {reason}")]
    Variant2 { reason: String },
}
```

---

## 🎨 Formatting Standards

### Lists

#### Unordered Lists

```markdown
- First item
- Second item
  - Nested item
  - Another nested item
- Third item
```

#### Ordered Lists

```markdown
1. First step
2. Second step
   1. Sub-step
   2. Another sub-step
3. Third step
```

#### Definition Lists

```markdown
**Term 1**: Definition of term 1
**Term 2**: Definition of term 2
```

### Tables

```markdown
| Column 1 | Column 2 | Column 3 |
| -------- | -------- | -------- |
| Data 1   | Data 2   | Data 3   |
| Data 4   | Data 5   | Data 6   |
```

### Blockquotes

```markdown
> **Note**: Important information that should be highlighted
>
> Additional context or explanation
```

### Code Blocks

#### Inline Code

```markdown
Use the `function_name()` function to perform the operation.
```

#### Fenced Code Blocks

````markdown
```rust
pub fn example() {
    println!("Hello, world!");
}
```
````

#### Syntax Highlighting

````markdown
```toml
# TOML configuration
[section]
key = "value"
```
````

```bash
# Shell commands
cargo build
```

```json
{
  "key": "value"
}
```

````

---

## 🔍 Review Checklist

### Content Review

- [ ] All technical details are accurate
- [ ] Code examples are complete and functional
- [ ] Cross-references are correct and up-to-date
- [ ] No redundant information with other specifications
- [ ] All required sections are present

### Style Review

- [ ] Follows naming conventions
- [ ] Uses consistent terminology
- [ ] Proper grammar and spelling
- [ ] Clear and concise language
- [ ] Appropriate code documentation

### Structure Review

- [ ] Proper document structure
- [ ] Consistent header hierarchy
- [ ] Logical flow of information
- [ ] Appropriate use of lists and tables
- [ ] Clear separation of concerns

### Integration Review

- [ ] References other specifications correctly
- [ ] No conflicts with architectural principles
- [ ] Consistent with overall design patterns
- [ ] Proper error handling patterns
- [ ] Appropriate configuration patterns

---

## 📚 Maintenance Guidelines

### Adding New Specifications

1. **Create the document** following the naming convention
2. **Add required frontmatter** with description
3. **Follow the document structure** outlined in this guide
4. **Review against this style guide** before finalizing
5. **Update cross-references** in related documents
6. **Add to specification index** if applicable

### Updating Existing Specifications

1. **Maintain consistency** with this style guide
2. **Update cross-references** when changing content
3. **Preserve architectural integrity** when making changes
4. **Document breaking changes** clearly
5. **Update related specifications** if necessary

### Version Control

- **Commit messages** should reference the specification being changed
- **Branch naming** should include the specification name
- **Review process** should include style guide compliance
- **Documentation** of changes should be clear and complete

---

## 🎯 Quality Metrics

### Consistency Metrics

- **Naming Consistency**: 100% adherence to naming conventions
- **Cross-Reference Accuracy**: All internal links are valid
- **Terminology Consistency**: Consistent use of defined terms
- **Formatting Consistency**: Uniform formatting across all documents

### Completeness Metrics

- **Required Sections**: All specifications have required sections
- **Code Examples**: All concepts have appropriate code examples
- **Error Handling**: All error scenarios are documented
- **Integration Points**: All component interactions are documented

### Clarity Metrics

- **Readability**: Clear and understandable language
- **Structure**: Logical organization of information
- **Examples**: Sufficient examples for complex concepts
- **Cross-References**: Clear navigation between related content

---

## 📖 Examples

### Good Specification Structure

```markdown
---
description: Task Management System Specification
alwaysApply: true
---

# Task Management System

This specification defines the task management system for Edda, providing comprehensive task creation, tracking, and management capabilities.

## Architecture Context

The Task Management System operates within the Core Engine Layer and integrates with:

- **Data Storage Layer**: For task persistence
- **CLI Interface Layer**: For task commands
- **AI Agent Integration Layer**: For AI agent interactions

## Core Concepts

### Task Definition

```rust
/// Represents a task in the system
#[derive(Debug, Clone)]
pub struct Task {
    /// Unique task identifier
    pub id: TaskId,
    /// Task title
    pub title: String,
    /// Task description
    pub description: Option<String>,
    /// Task status
    pub status: TaskStatus,
}
````

## Implementation Details

[Implementation details with code examples]

## Integration Points

[Integration details with other components]

## Configuration

[Configuration options and patterns]

## Error Handling

[Error scenarios and handling strategies]

## Performance Considerations

[Performance implications and optimizations]

## Security Considerations

[Security implications and best practices]

## Testing Strategy

[Testing approaches and requirements]

````

### Good Code Documentation

```rust
/// Creates a new task in the system
///
/// # Arguments
/// * `title` - The title of the task
/// * `description` - Optional description of the task
/// * `priority` - The priority level of the task
///
/// # Returns
/// The created task with a unique identifier
///
/// # Errors
/// Returns an error if the task cannot be created due to validation
/// or storage issues
///
/// # Examples
/// ```rust
/// let task = create_task("Example Task", Some("Description"), Priority::Medium)?;
/// ```
pub fn create_task(
    title: String,
    description: Option<String>,
    priority: Priority,
) -> Result<Task, TaskError> {
    // Implementation
}
````

---

This style guide ensures that all Edda specification documents maintain high quality, consistency, and usability. All contributors should follow these standards when creating or updating specification documents.

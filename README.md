# Shing ⚔️

A lightweight AI prompt template library for Rust.

## Features

- 🎯 Simple and intuitive API
- 📝 Template variable substitution
- 🔄 Multiple output formats (text/JSON)
- 🚀 Zero dependencies beyond serde
- ⚡ Fast and lightweight

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shing = "0.1"
```

## Quick Start

```rust
use shing::PromptBuilder;

// Basic usage
let prompt = PromptBuilder::new()
    .system("You are a helpful assistant.")
    .user("Hello, {{name}}!")
    .var("name", "World")
    .build();

println!("{}", prompt);
```

## API

### PromptBuilder

The main builder for constructing prompts:

```rust
let prompt = PromptBuilder::new()
    .system("System message")
    .user("User message with {{var}}")
    .assistant("Optional assistant message")
    .var("var", "value")
    .build();
```

### JSON Output

For use with LLM APIs:

```rust
let json = PromptBuilder::new()
    .system("You are helpful.")
    .user("Hello!")
    .build_json()?;
// Returns: {"messages": [{"role": "system", "content": "..."}, ...]}
```

### Quick Function

```rust
use shing::prompt;

let p = prompt("You are helpful.", "Hello!");
```

## Why Shing?

"Shing" represents the sharp, clean approach to prompt engineering - like the sound of a sword being drawn. The library focuses on simplicity and performance.

## License

MIT OR Apache-2.0

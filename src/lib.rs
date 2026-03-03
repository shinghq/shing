//! # Shing - A Lightweight AI Prompt Template Library
//!
//! Shing provides a simple way to build and manage AI prompts with template support.
//!
//! # Example
//!
//! ```
//! use shing::PromptBuilder;
//!
//! let prompt = PromptBuilder::new()
//!     .system("You are a helpful assistant.")
//!     .user("Hello, {{name}}!")
//!     .var("name", "World")
//!     .build();
//!
//! assert_eq!(prompt, "System: You are a helpful assistant.\nUser: Hello, World!");
//! ```

use std::collections::HashMap;

/// Error type for Shing operations
#[derive(Debug, Clone)]
pub enum Error {
    /// Variable not found in template
    MissingVariable(String),
    /// Invalid template syntax
    InvalidSyntax(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingVariable(name) => write!(f, "Missing variable: {}", name),
            Error::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for Shing operations
pub type Result<T> = std::result::Result<T, Error>;

/// Builder for constructing AI prompts
#[derive(Debug, Clone)]
pub struct PromptBuilder {
    system: Option<String>,
    user: Option<String>,
    assistant: Option<String>,
    variables: HashMap<String, String>,
}

impl PromptBuilder {
    /// Create a new PromptBuilder
    pub fn new() -> Self {
        Self {
            system: None,
            user: None,
            assistant: None,
            variables: HashMap::new(),
        }
    }

    /// Set the system message
    pub fn system(mut self, msg: impl Into<String>) -> Self {
        self.system = Some(msg.into());
        self
    }

    /// Set the user message
    pub fn user(mut self, msg: impl Into<String>) -> Self {
        self.user = Some(msg.into());
        self
    }

    /// Set the assistant message
    pub fn assistant(mut self, msg: impl Into<String>) -> Self {
        self.assistant = Some(msg.into());
        self
    }

    /// Add a variable for template substitution
    pub fn var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(key.into(), value.into());
        self
    }

    /// Add multiple variables
    pub fn vars(mut self, vars: HashMap<String, String>) -> Self {
        self.variables.extend(vars);
        self
    }

    /// Build the final prompt string
    pub fn build(self) -> String {
        let vars = &self.variables;
        let mut parts = Vec::new();

        if let Some(ref sys) = self.system {
            parts.push(format!("System: {}", interpolate(sys, vars)));
        }

        if let Some(ref user) = self.user {
            parts.push(format!("User: {}", interpolate(user, vars)));
        }

        if let Some(ref assistant) = self.assistant {
            parts.push(format!("Assistant: {}", interpolate(assistant, vars)));
        }

        parts.join("\n")
    }

    /// Build as JSON format (for API calls)
    pub fn build_json(self) -> Result<String> {
        let vars = &self.variables;
        let messages: Vec<serde_json::Value> = vec![
            self.system.as_ref().map(|s| serde_json::json!({
                "role": "system",
                "content": interpolate(s, vars)
            })),
            self.user.as_ref().map(|s| serde_json::json!({
                "role": "user",
                "content": interpolate(s, vars)
            })),
            self.assistant.as_ref().map(|s| serde_json::json!({
                "role": "assistant",
                "content": interpolate(s, vars)
            })),
        ].into_iter().flatten().collect();

        serde_json::to_string_pretty(&serde_json::json!({"messages": messages}))
            .map_err(|e| Error::InvalidSyntax(e.to_string()))
    }
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn interpolate(template: &str, vars: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        result = result.replace(&format!("{{{{{}}}}}", key), value);
    }
    result
}

/// Quick function to create a simple prompt
pub fn prompt(system: &str, user: &str) -> String {
    PromptBuilder::new()
        .system(system)
        .user(user)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_prompt() {
        let p = PromptBuilder::new()
            .system("You are helpful.")
            .user("Hi!")
            .build();
        
        assert!(p.contains("System: You are helpful."));
        assert!(p.contains("User: Hi!"));
    }

    #[test]
    fn test_variable_substitution() {
        let p = PromptBuilder::new()
            .user("Hello, {{name}}!")
            .var("name", "World")
            .build();
        
        assert_eq!(p, "User: Hello, World!");
    }

    #[test]
    fn test_json_output() {
        let p = PromptBuilder::new()
            .system("Test")
            .user("Hello")
            .build_json()
            .unwrap();
        
        assert!(p.contains("\"role\": \"system\""));
        assert!(p.contains("\"role\": \"user\""));
    }
}

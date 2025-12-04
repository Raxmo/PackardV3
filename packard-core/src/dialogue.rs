use regex::Regex;

#[derive(Debug, Clone)]
pub struct DialogueLine {
    pub character: String,
    pub text: String,
}

/// Extract dialogue lines from scene content
/// Format: **Character Name**: "dialogue text"
pub fn extract_dialogue(content: &str) -> Vec<DialogueLine> {
    let re = Regex::new(r"\*\*([^*]+)\*\*:\s*(.+?)(?:\n|$)").unwrap();
    let mut lines = Vec::new();

    for cap in re.captures_iter(content) {
        let character = cap.get(1).unwrap().as_str().to_string();
        let mut text = cap.get(2).unwrap().as_str().to_string();
        
        // Remove quotes if present
        if (text.starts_with('"') && text.ends_with('"')) ||
           (text.starts_with('\'') && text.ends_with('\'')) {
            text = text[1..text.len()-1].to_string();
        }
        
        lines.push(DialogueLine { character, text });
    }

    lines
}

/// Remove dialogue lines from content
pub fn strip_dialogue(content: &str) -> String {
    let re = Regex::new(r"\*\*[^*]+\*\*:\s*.+?(?:\n|$)").unwrap();
    re.replace_all(content, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_dialogue() {
        let content = r#"Some scene text.

**Alice**: "Hello there!"
**Bob**: "How are you?"

More text."#;

        let lines = extract_dialogue(content);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].character, "Alice");
        assert_eq!(lines[0].text, "Hello there!");
        assert_eq!(lines[1].character, "Bob");
        assert_eq!(lines[1].text, "How are you?");
    }

    #[test]
    fn test_strip_dialogue() {
        let content = r#"Some text.
**Alice**: "Hello!"
More text."#;

        let stripped = strip_dialogue(content);
        assert!(!stripped.contains("**Alice**"));
        assert!(stripped.contains("Some text"));
    }
}

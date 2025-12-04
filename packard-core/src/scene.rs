use regex::Regex;
use crate::effects::Effect;

#[derive(Debug, Clone)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub content: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone)]
pub struct Choice {
    pub target: String,
    pub label: String,
    pub effects: Vec<Effect>,
}

impl Scene {
    pub fn from_markdown(id: String, content: &str) -> Result<Self, String> {
        // Split frontmatter from content
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        
        let (frontmatter, body) = if parts.len() == 3 {
            (parts[1], parts[2])
        } else {
            ("", content)
        };

        // Parse YAML frontmatter
        let mut title = id.clone();
        if !frontmatter.is_empty() {
            if let Ok(data) = serde_yaml::from_str::<serde_yaml::Value>(frontmatter) {
                if let Some(title_val) = data.get("title") {
                    if let Some(title_str) = title_val.as_str() {
                        title = title_str.to_string();
                    }
                }
            }
        }

        // Parse wikilinks for choices: [[target|label]](effects)
        // Pattern: [[target|label]] or [[target|label]](effect1; effect2)
        let wikilink_re = Regex::new(r"\[\[([^\]|]+)\|([^\]]+)\]\](?:\(([^)]*)\))?").unwrap();
        let mut choices = Vec::new();

        for cap in wikilink_re.captures_iter(body) {
            let target = cap.get(1).unwrap().as_str().to_string();
            let label = cap.get(2).unwrap().as_str().to_string();
            
            let effects = if let Some(effects_str) = cap.get(3) {
                crate::effects::parse_effects(effects_str.as_str()).unwrap_or_default()
            } else {
                Vec::new()
            };

            choices.push(Choice { target, label, effects });
        }

        Ok(Scene {
            id,
            title,
            content: body.to_string(),
            choices,
        })
    }
}

use regex::Regex;
use crate::effects::Effect;
use crate::conditions::Condition;
use crate::dialogue::DialogueLine;

#[derive(Debug, Clone)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub content: String,
    pub choices: Vec<Choice>,
    pub dialogue: Vec<DialogueLine>,
}

#[derive(Debug, Clone)]
pub struct Choice {
    pub target: String,
    pub label: String,
    pub effects: Vec<Effect>,
    pub condition: Option<Condition>,
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

        // Parse all wikilinks and conditionals: {if: condition}[[target|label]](effects) or [[target|label]](effects)
        let choice_re = Regex::new(r"\{if:\s*([^}]+)\}?\[\[([^\]|]+)\|([^\]]+)\]\](?:\(([^)]*)\))?").unwrap();
        let mut choices = Vec::new();
        let mut processed_positions = std::collections::HashSet::new();

        for cap in choice_re.captures_iter(body) {
            let condition_str = cap.get(1).as_ref().map(|c| c.as_str());
            let target = cap.get(2).unwrap().as_str().to_string();
            let label = cap.get(3).unwrap().as_str().to_string();
            
            let condition = condition_str.and_then(|s| crate::conditions::parse_condition(s).ok());
            
            let effects = if let Some(effects_str) = cap.get(4) {
                crate::effects::parse_effects(effects_str.as_str()).unwrap_or_default()
            } else {
                Vec::new()
            };

            // Track position to avoid duplicates
            if let Some(pos) = cap.get(0) {
                processed_positions.insert(pos.start());
            }

            choices.push(Choice { 
                target, 
                label, 
                effects,
                condition,
            });
        }

        // Also parse simple unconditional wikilinks: [[target|label]](effects)
        let wikilink_re = Regex::new(r"\[\[([^\]|]+)\|([^\]]+)\]\](?:\(([^)]*)\))?").unwrap();
        
        for cap in wikilink_re.captures_iter(body) {
            // Skip if this was already captured by the conditional regex
            if let Some(pos) = cap.get(0) {
                let text = pos.as_str();
                if text.contains("{if:") {
                    continue;
                }
            }
            
            let target = cap.get(1).unwrap().as_str().to_string();
            let label = cap.get(2).unwrap().as_str().to_string();
            
            let effects = if let Some(effects_str) = cap.get(3) {
                crate::effects::parse_effects(effects_str.as_str()).unwrap_or_default()
            } else {
                Vec::new()
            };

            choices.push(Choice { 
                target, 
                label, 
                effects,
                condition: None,
            });
        }

        // Extract dialogue from content
        let dialogue = crate::dialogue::extract_dialogue(body);

        Ok(Scene {
            id,
            title,
            content: body.to_string(),
            choices,
            dialogue,
        })
    }
}

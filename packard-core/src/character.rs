use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub description: String,
    pub properties: HashMap<String, serde_yaml::Value>,
}

impl Character {
    pub fn from_markdown(id: String, content: &str) -> Result<Self, String> {
        // Split frontmatter from content
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        
        let (frontmatter, body) = if parts.len() == 3 {
            (parts[1], parts[2])
        } else {
            ("", content)
        };

        // Parse YAML frontmatter
        let mut name = id.clone();
        let mut properties = HashMap::new();

        if !frontmatter.is_empty() {
            if let Ok(data) = serde_yaml::from_str::<serde_yaml::Value>(frontmatter) {
                // Extract name
                if let Some(name_val) = data.get("name") {
                    if let Some(name_str) = name_val.as_str() {
                        name = name_str.to_string();
                    }
                }
                
                // Store all properties for later access
                if let Some(obj) = data.as_mapping() {
                    for (key, val) in obj {
                        if let Some(key_str) = key.as_str() {
                            if key_str != "name" {
                                properties.insert(key_str.to_string(), val.clone());
                            }
                        }
                    }
                }
            }
        }

        let description = body.trim().to_string();

        Ok(Character {
            id,
            name,
            description,
            properties,
        })
    }

    pub fn get_property(&self, key: &str) -> Option<&serde_yaml::Value> {
        self.properties.get(key)
    }
}

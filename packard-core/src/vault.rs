use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::scene::Scene;
use crate::character::Character;

pub struct Vault {
    pub scenes: HashMap<String, Scene>,
    pub characters: HashMap<String, Character>,
}

impl Vault {
    pub fn load(path: &str) -> Result<Self, String> {
        let mut scenes = HashMap::new();
        let mut characters = HashMap::new();
        let vault_path = Path::new(path);

        if !vault_path.exists() {
            return Err(format!("Vault path does not exist: {}", path));
        }

        // Walk through all markdown files in the vault
        for entry in WalkDir::new(vault_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        {
            let file_path = entry.path();
            
            // Skip .obsidian folder
            if file_path.components().any(|c| c.as_os_str() == ".obsidian") {
                continue;
            }

            // Read file
            let content = fs::read_to_string(file_path)
                .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

            // Get ID from filename (without .md)
            let id = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or("Invalid filename")?
                .to_string();

            // Check if file is in characters folder
            let is_character = file_path
                .components()
                .any(|c| c.as_os_str() == "characters");

            if is_character {
                let character = Character::from_markdown(id.clone(), &content)?;
                characters.insert(id, character);
            } else {
                let scene = Scene::from_markdown(id.clone(), &content)?;
                scenes.insert(id, scene);
            }
        }

        if scenes.is_empty() {
            return Err("No markdown files found in vault".to_string());
        }

        Ok(Vault { scenes, characters })
    }

    pub fn get_scene(&self, id: &str) -> Option<&Scene> {
        self.scenes.get(id)
    }

    pub fn list_scenes(&self) -> Vec<String> {
        let mut ids: Vec<_> = self.scenes.keys().cloned().collect();
        ids.sort();
        ids
    }

    pub fn get_character(&self, id: &str) -> Option<&Character> {
        self.characters.get(id)
    }

    pub fn list_characters(&self) -> Vec<String> {
        let mut ids: Vec<_> = self.characters.keys().cloned().collect();
        ids.sort();
        ids
    }
}

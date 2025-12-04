use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use crate::scene::Scene;

pub struct Vault {
    pub scenes: HashMap<String, Scene>,
}

impl Vault {
    pub fn load(path: &str) -> Result<Self, String> {
        let mut scenes = HashMap::new();
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

            // Get scene ID from filename (without .md)
            let scene_id = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or("Invalid filename")?
                .to_string();

            // Parse scene from markdown
            let scene = Scene::from_markdown(scene_id.clone(), &content)?;
            scenes.insert(scene_id, scene);
        }

        if scenes.is_empty() {
            return Err("No markdown files found in vault".to_string());
        }

        Ok(Vault { scenes })
    }

    pub fn get_scene(&self, id: &str) -> Option<&Scene> {
        self.scenes.get(id)
    }

    pub fn list_scenes(&self) -> Vec<String> {
        let mut ids: Vec<_> = self.scenes.keys().cloned().collect();
        ids.sort();
        ids
    }
}

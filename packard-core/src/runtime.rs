use crate::vault::Vault;
use crate::scene::Scene;

pub struct Runtime {
    vault: Vault,
    current_scene_id: String,
}

impl Runtime {
    pub fn new(vault: Vault, start_scene: &str) -> Result<Self, String> {
        if !vault.get_scene(start_scene).is_some() {
            return Err(format!("Start scene '{}' not found", start_scene));
        }

        Ok(Runtime {
            vault,
            current_scene_id: start_scene.to_string(),
        })
    }

    pub fn current_scene(&self) -> &Scene {
        self.vault.get_scene(&self.current_scene_id).unwrap()
    }

    pub fn current_scene_id(&self) -> &str {
        &self.current_scene_id
    }

    pub fn choose(&mut self, choice_index: usize) -> Result<(), String> {
        let scene = self.current_scene();
        
        if choice_index >= scene.choices.len() {
            return Err(format!("Invalid choice: {}", choice_index));
        }

        let next_id = scene.choices[choice_index].target.clone();
        
        if !self.vault.get_scene(&next_id).is_some() {
            return Err(format!("Scene '{}' not found", next_id));
        }

        self.current_scene_id = next_id;
        Ok(())
    }
}

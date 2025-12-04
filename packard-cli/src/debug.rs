use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use chrono::Local;

pub struct DebugLogger {
    file: Option<Mutex<std::fs::File>>,
}

impl DebugLogger {
    pub fn new(path: Option<&str>) -> std::io::Result<Self> {
        let file = if let Some(p) = path {
            let f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(p)?;
            Some(Mutex::new(f))
        } else {
            None
        };

        Ok(DebugLogger { file })
    }

    pub fn log(&self, message: &str) {
        if let Some(ref f) = self.file {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            if let Ok(mut file) = f.lock() {
                let _ = writeln!(file, "[{}] {}", timestamp, message);
            }
        }
    }

    pub fn log_state(&self, runtime: &packard_core::Runtime) {
        self.log("=== STATE ===");
        for (key, value) in &runtime.state().variables {
            self.log(&format!("  {}: {:?}", key, value));
        }
    }

    pub fn log_scene(&self, scene_id: &str) {
        self.log(&format!("SCENE CHANGE -> {}", scene_id));
    }

    pub fn log_choice(&self, choice_idx: usize, label: &str) {
        self.log(&format!("CHOICE MADE -> {} ({})", choice_idx + 1, label));
    }

    pub fn log_effects(&self, effects: &[packard_core::Effect]) {
        if !effects.is_empty() {
            self.log("EFFECTS APPLIED:");
            for effect in effects {
                self.log(&format!("  {} {} {}", effect.variable, effect.operation, effect.value));
            }
        }
    }
}

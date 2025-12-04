use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Effect {
    pub variable: String,
    pub operation: String, // "=", "+=", "-=", etc.
    pub value: String,
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub variables: HashMap<String, serde_yaml::Value>,
}

impl State {
    pub fn new() -> Self {
        State {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: serde_yaml::Value) {
        self.variables.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&serde_yaml::Value> {
        self.variables.get(key)
    }

    pub fn apply_effects(&mut self, effects: &[Effect]) -> Result<(), String> {
        for effect in effects {
            self.apply_effect(effect)?;
        }
        Ok(())
    }

    fn apply_effect(&mut self, effect: &Effect) -> Result<(), String> {
        match effect.operation.as_str() {
            "=" => {
                // Parse value as number or string
                if let Ok(num) = effect.value.parse::<i64>() {
                    self.set(&effect.variable, serde_yaml::Value::Number(num.into()));
                } else if effect.value == "true" {
                    self.set(&effect.variable, serde_yaml::Value::Bool(true));
                } else if effect.value == "false" {
                    self.set(&effect.variable, serde_yaml::Value::Bool(false));
                } else {
                    self.set(&effect.variable, serde_yaml::Value::String(effect.value.clone()));
                }
            }
            "+=" => {
                let current = self.get(&effect.variable)
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let delta: i64 = effect.value.parse()
                    .map_err(|_| format!("Invalid number for +=: {}", effect.value))?;
                self.set(&effect.variable, serde_yaml::Value::Number((current + delta).into()));
            }
            "-=" => {
                let current = self.get(&effect.variable)
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let delta: i64 = effect.value.parse()
                    .map_err(|_| format!("Invalid number for -=: {}", effect.value))?;
                self.set(&effect.variable, serde_yaml::Value::Number((current - delta).into()));
            }
            _ => return Err(format!("Unknown operation: {}", effect.operation)),
        }
        Ok(())
    }
}

pub fn parse_effects(effects_str: &str) -> Result<Vec<Effect>, String> {
    let mut effects = Vec::new();
    
    // Split by semicolon for multiple effects
    for effect_expr in effects_str.split(';') {
        let effect_expr = effect_expr.trim();
        if effect_expr.is_empty() {
            continue;
        }

        // Match pattern: variable (op) value
        let re = Regex::new(r"^([a-z_][a-z0-9_.]*)\s*(\+=|-=|=)\s*(.+)$")
            .map_err(|e| format!("Regex error: {}", e))?;

        if let Some(cap) = re.captures(effect_expr) {
            let variable = cap.get(1).unwrap().as_str().to_string();
            let operation = cap.get(2).unwrap().as_str().to_string();
            let value = cap.get(3).unwrap().as_str().trim().to_string();

            effects.push(Effect {
                variable,
                operation,
                value,
            });
        } else {
            return Err(format!("Invalid effect syntax: {}", effect_expr));
        }
    }

    Ok(effects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_effect() {
        let effects = parse_effects("player.trust = 50").unwrap();
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].variable, "player.trust");
        assert_eq!(effects[0].operation, "=");
        assert_eq!(effects[0].value, "50");
    }

    #[test]
    fn test_parse_multiple_effects() {
        let effects = parse_effects("player.trust += 10; player.kindness = 5").unwrap();
        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0].variable, "player.trust");
        assert_eq!(effects[0].operation, "+=");
        assert_eq!(effects[1].variable, "player.kindness");
        assert_eq!(effects[1].operation, "=");
    }

    #[test]
    fn test_apply_effects() {
        let mut state = State::new();
        let effects = parse_effects("player.trust = 50; player.kindness += 10").unwrap();
        
        state.apply_effects(&effects).unwrap();
        
        assert_eq!(state.get("player.trust").unwrap().as_i64(), Some(50));
        assert_eq!(state.get("player.kindness").unwrap().as_i64(), Some(10));
    }

    #[test]
    fn test_apply_effects_increment() {
        let mut state = State::new();
        state.set("counter", serde_yaml::Value::Number(5.into()));
        
        let effects = parse_effects("counter += 3").unwrap();
        state.apply_effects(&effects).unwrap();
        
        assert_eq!(state.get("counter").unwrap().as_i64(), Some(8));
    }
}

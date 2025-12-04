use regex::Regex;
use crate::effects::State;

#[derive(Debug, Clone)]
pub struct Condition {
    pub variable: String,
    pub operator: String, // ">", "<", ">=", "<=", "==", "!="
    pub value: String,
}

impl Condition {
    pub fn evaluate(&self, state: &State) -> Result<bool, String> {
        let var_value = state
            .get(&self.variable)
            .ok_or(format!("Variable '{}' not found in state", self.variable))?;

        // Get numeric value from state
        let left = var_value
            .as_i64()
            .ok_or(format!("Variable '{}' is not a number", self.variable))?;

        // Parse right side as number
        let right: i64 = self.value.parse()
            .map_err(|_| format!("Cannot parse '{}' as number", self.value))?;

        let result = match self.operator.as_str() {
            ">" => left > right,
            "<" => left < right,
            ">=" => left >= right,
            "<=" => left <= right,
            "==" => left == right,
            "!=" => left != right,
            _ => return Err(format!("Unknown operator: {}", self.operator)),
        };

        Ok(result)
    }
}

pub fn parse_condition(condition_str: &str) -> Result<Condition, String> {
    // Pattern: variable (op) value
    let re = Regex::new(r"^([a-z_][a-z0-9_.]*)\s*(>=|<=|==|!=|>|<)\s*(.+)$")
        .map_err(|e| format!("Regex error: {}", e))?;

    if let Some(cap) = re.captures(condition_str.trim()) {
        let variable = cap.get(1).unwrap().as_str().to_string();
        let operator = cap.get(2).unwrap().as_str().to_string();
        let value = cap.get(3).unwrap().as_str().trim().to_string();

        Ok(Condition {
            variable,
            operator,
            value,
        })
    } else {
        Err(format!("Invalid condition syntax: {}", condition_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_condition() {
        let cond = parse_condition("player.health > 50").unwrap();
        assert_eq!(cond.variable, "player.health");
        assert_eq!(cond.operator, ">");
        assert_eq!(cond.value, "50");
    }

    #[test]
    fn test_parse_condition_equals() {
        let cond = parse_condition("player.trust == 100").unwrap();
        assert_eq!(cond.operator, "==");
    }

    #[test]
    fn test_evaluate_condition_true() {
        let mut state = State::new();
        state.set("player.health", serde_yaml::Value::Number(100.into()));
        
        let cond = parse_condition("player.health > 50").unwrap();
        assert_eq!(cond.evaluate(&state).unwrap(), true);
    }

    #[test]
    fn test_evaluate_condition_false() {
        let mut state = State::new();
        state.set("player.health", serde_yaml::Value::Number(30.into()));
        
        let cond = parse_condition("player.health > 50").unwrap();
        assert_eq!(cond.evaluate(&state).unwrap(), false);
    }

    #[test]
    fn test_evaluate_condition_equal() {
        let mut state = State::new();
        state.set("player.trust", serde_yaml::Value::Number(100.into()));
        
        let cond = parse_condition("player.trust == 100").unwrap();
        assert_eq!(cond.evaluate(&state).unwrap(), true);
    }
}

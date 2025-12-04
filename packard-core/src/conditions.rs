use regex::Regex;
use crate::effects::State;

#[derive(Debug, Clone)]
pub struct SimpleCondition {
    pub variable: String,
    pub operator: String, // ">", "<", ">=", "<=", "==", "!="
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum Condition {
    Simple(SimpleCondition),
    Compound(Vec<(Option<String>, SimpleCondition)>), // (operator, condition) where first operator is None
}

impl SimpleCondition {
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

impl Condition {
    pub fn evaluate(&self, state: &State) -> Result<bool, String> {
        match self {
            Condition::Simple(cond) => cond.evaluate(state),
            Condition::Compound(conditions) => {
                if conditions.is_empty() {
                    return Ok(true);
                }

                let mut result = conditions[0].1.evaluate(state)?;

                for (op, cond) in &conditions[1..] {
                    let cond_result = cond.evaluate(state)?;
                    match op.as_ref().map(|s| s.as_str()) {
                        Some("AND") => result = result && cond_result,
                        Some("OR") => result = result || cond_result,
                        _ => return Err("Unknown logical operator".to_string()),
                    }
                }

                Ok(result)
            }
        }
    }
}

pub fn parse_condition(condition_str: &str) -> Result<Condition, String> {
    let condition_str = condition_str.trim();

    // Check if it's a compound condition (contains AND or OR)
    if condition_str.contains(" AND ") || condition_str.contains(" OR ") {
        parse_compound_condition(condition_str)
    } else {
        parse_simple_condition(condition_str).map(Condition::Simple)
    }
}

fn parse_simple_condition(condition_str: &str) -> Result<SimpleCondition, String> {
    // Pattern: variable (op) value
    let re = Regex::new(r"^([a-z_][a-z0-9_.]*)\s*(>=|<=|==|!=|>|<)\s*(.+)$")
        .map_err(|e| format!("Regex error: {}", e))?;

    if let Some(cap) = re.captures(condition_str.trim()) {
        let variable = cap.get(1).unwrap().as_str().to_string();
        let operator = cap.get(2).unwrap().as_str().to_string();
        let value = cap.get(3).unwrap().as_str().trim().to_string();

        Ok(SimpleCondition {
            variable,
            operator,
            value,
        })
    } else {
        Err(format!("Invalid condition syntax: {}", condition_str))
    }
}

fn parse_compound_condition(condition_str: &str) -> Result<Condition, String> {
    let mut conditions = Vec::new();

    // Split by AND and OR while preserving the operators
    let parts: Vec<&str> = condition_str.split(" AND ").collect();
    
    if parts.len() > 1 {
        // Contains AND operators
        for (i, part) in parts.iter().enumerate() {
            let simple = parse_simple_condition(part)?;
            if i == 0 {
                conditions.push((None, simple));
            } else {
                conditions.push((Some("AND".to_string()), simple));
            }
        }
    } else {
        // Try splitting by OR
        let parts: Vec<&str> = condition_str.split(" OR ").collect();
        if parts.len() > 1 {
            for (i, part) in parts.iter().enumerate() {
                let simple = parse_simple_condition(part)?;
                if i == 0 {
                    conditions.push((None, simple));
                } else {
                    conditions.push((Some("OR".to_string()), simple));
                }
            }
        } else {
            // Single condition
            return parse_simple_condition(condition_str).map(Condition::Simple);
        }
    }

    Ok(Condition::Compound(conditions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_condition() {
        let cond = parse_simple_condition("player.health > 50").unwrap();
        assert_eq!(cond.variable, "player.health");
        assert_eq!(cond.operator, ">");
        assert_eq!(cond.value, "50");
    }

    #[test]
    fn test_evaluate_simple_condition_true() {
        let mut state = State::new();
        state.set("player.health", serde_yaml::Value::Number(100.into()));
        
        let cond = parse_condition("player.health > 50").unwrap();
        assert_eq!(cond.evaluate(&state).unwrap(), true);
    }

    #[test]
    fn test_evaluate_simple_condition_false() {
        let mut state = State::new();
        state.set("player.health", serde_yaml::Value::Number(30.into()));
        
        let cond = parse_condition("player.health > 50").unwrap();
        assert_eq!(cond.evaluate(&state).unwrap(), false);
    }

    #[test]
    fn test_parse_compound_condition_and() {
        let cond = parse_condition("player.health > 50 AND player.trust >= 30").unwrap();
        match cond {
            Condition::Compound(conditions) => {
                assert_eq!(conditions.len(), 2);
                assert_eq!(conditions[0].0, None);
                assert_eq!(conditions[1].0, Some("AND".to_string()));
            }
            _ => panic!("Expected compound condition"),
        }
    }

    #[test]
    fn test_evaluate_compound_condition_and_true() {
        let mut state = State::new();
        state.set("player.health", serde_yaml::Value::Number(100.into()));
        state.set("player.trust", serde_yaml::Value::Number(50.into()));
        
        let cond = parse_condition("player.health > 50 AND player.trust >= 30").unwrap();
        assert_eq!(cond.evaluate(&state).unwrap(), true);
    }

    #[test]
    fn test_evaluate_compound_condition_and_false() {
        let mut state = State::new();
        state.set("player.health", serde_yaml::Value::Number(30.into()));
        state.set("player.trust", serde_yaml::Value::Number(50.into()));
        
        let cond = parse_condition("player.health > 50 AND player.trust >= 30").unwrap();
        assert_eq!(cond.evaluate(&state).unwrap(), false);
    }

    #[test]
    fn test_evaluate_compound_condition_or_true() {
        let mut state = State::new();
        state.set("player.health", serde_yaml::Value::Number(30.into()));
        state.set("player.trust", serde_yaml::Value::Number(50.into()));
        
        let cond = parse_condition("player.health > 50 OR player.trust >= 30").unwrap();
        assert_eq!(cond.evaluate(&state).unwrap(), true);
    }

    #[test]
    fn test_evaluate_compound_condition_or_false() {
        let mut state = State::new();
        state.set("player.health", serde_yaml::Value::Number(30.into()));
        state.set("player.trust", serde_yaml::Value::Number(20.into()));
        
        let cond = parse_condition("player.health > 50 OR player.trust >= 30").unwrap();
        assert_eq!(cond.evaluate(&state).unwrap(), false);
    }
}

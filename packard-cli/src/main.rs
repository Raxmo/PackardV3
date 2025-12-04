use packard_core::{Vault, Runtime};
use std::io::{self, Write};
use std::env;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn strip_wikilinks(content: &str) -> String {
    // Remove wikilink syntax with optional effects: [[target|label]](effects)
    let re = regex::Regex::new(r"\[\[([^\]|]+)\|([^\]]+)\]\](?:\([^)]*\))?").unwrap();
    re.replace_all(content, "").to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let vault_path = if args.len() > 1 {
        &args[1]
    } else {
        println!("Usage: packard <vault_path>");
        return;
    };

    // Load the vault
    let vault = match Vault::load(vault_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error loading vault: {}", e);
            return;
        }
    };

    // Create runtime starting from "start" scene
    let mut runtime = match Runtime::new(vault, "start") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error initializing runtime: {}", e);
            return;
        }
    };

    clear_screen();

    // Main loop
    loop {
        let scene = runtime.current_scene();
        
        // Show title and wait for input
        println!("{}", scene.title);
        println!();
        print!("Press Enter to continue...");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        clear_screen();
        
        // Show content (without wikilinks)
        let clean_content = strip_wikilinks(&scene.content);
        println!("{}", clean_content);
        
        // Get available choices based on conditions
        let available_choices = runtime.available_choices();

        if available_choices.is_empty() {
            println!("\n[End of narrative]");
            break;
        }

        // Show state
        println!("\n{}", "-".repeat(40));
        println!("State:");
        for (key, value) in &runtime.state().variables {
            println!("  {}: {:?}", key, value);
        }

        // Show choices
        println!("\n{}", "-".repeat(40));
        for (display_idx, (_orig_idx, choice)) in available_choices.iter().enumerate() {
            print!("{}. {}", display_idx + 1, choice.label);
            if let Some(cond) = &choice.condition {
                print!(" {{if: {} {} {}}}", cond.variable, cond.operator, cond.value);
            }
            if !choice.effects.is_empty() {
                print!(" [", );
                for (j, effect) in choice.effects.iter().enumerate() {
                    if j > 0 { print!("; "); }
                    print!("{} {} {}", effect.variable, effect.operation, effect.value);
                }
                print!("]");
            }
            println!();
        }

        // Get user input
        print!("\nSelect choice (1-{}): ", available_choices.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let choice_idx = match input.trim().parse::<usize>() {
            Ok(n) if n > 0 && n <= available_choices.len() => n - 1,
            _ => {
                println!("Invalid choice. Try again.");
                continue;
            }
        };

        // Get the original index of the choice
        let (orig_idx, _) = available_choices[choice_idx];
        let choice = orig_idx;

        if let Err(e) = runtime.choose(choice) {
            eprintln!("Error: {}", e);
            break;
        }
        
        clear_screen();
    }
}

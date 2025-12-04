use packard_core::{Vault, Runtime};
use std::io::{self, Write};
use std::env;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn strip_wikilinks(content: &str) -> String {
    // Remove wikilink syntax entirely
    let re = regex::Regex::new(r"\[\[([^\]|]+)\|([^\]]+)\]\]").unwrap();
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
        
        if scene.choices.is_empty() {
            println!("\n[End of narrative]");
            break;
        }

        // Show choices
        println!("\n{}", "-".repeat(40));
        for (i, choice) in scene.choices.iter().enumerate() {
            println!("{}. {}", i + 1, choice.label);
        }

        // Get user input
        print!("\nSelect choice (1-{}): ", scene.choices.len());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice = match input.trim().parse::<usize>() {
            Ok(n) if n > 0 && n <= scene.choices.len() => n - 1,
            _ => {
                println!("Invalid choice");
                continue;
            }
        };

        if let Err(e) = runtime.choose(choice) {
            eprintln!("Error: {}", e);
            break;
        }
        
        clear_screen();
    }
}

use packard_core::{Vault, Runtime};
use std::io::{self, Write};
use std::env;

mod debug;
use debug::DebugLogger;

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
    
    let mut vault_path = "";
    let mut debug_log = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--debug" => {
                if i + 1 < args.len() {
                    debug_log = Some(args[i + 1].as_str());
                    i += 2;
                } else {
                    eprintln!("Error: -d requires a file path");
                    return;
                }
            }
            _ => {
                vault_path = &args[i];
                i += 1;
            }
        }
    }

    if vault_path.is_empty() {
        println!("Usage: packard [OPTIONS] <vault_path>");
        println!("Options:");
        println!("  -d, --debug <file>  Log debug information to file");
        return;
    }

    let logger = match DebugLogger::new(debug_log) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error creating debug log: {}", e);
            return;
        }
    };

    // Load the vault
    logger.log(&format!("Loading vault: {}", vault_path));
    let vault = match Vault::load(vault_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error loading vault: {}", e);
            return;
        }
    };

    logger.log(&format!("Vault loaded. Scenes: {:?}", vault.list_scenes()));

    // Create runtime starting from "start" scene
    let mut runtime = match Runtime::new(vault, "start") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error initializing runtime: {}", e);
            return;
        }
    };

    logger.log_scene("start");
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
        
        // Show content (without wikilinks and dialogue)
        let clean_content = strip_wikilinks(&scene.content);
        let clean_content = packard_core::dialogue::strip_dialogue(&clean_content);
        println!("{}", clean_content);

        // Show dialogue separately
        if !scene.dialogue.is_empty() {
            println!("\n{}", "-".repeat(40));
            for line in &scene.dialogue {
                println!("**{}**: \"{}\"", line.character, line.text);
            }
        }
        
        // Get available choices based on conditions
        let available_choices = runtime.available_choices();

        if available_choices.is_empty() {
            println!("\n[End of narrative]");
            break;
        }

        // Show choices
        println!();
        for (display_idx, (_orig_idx, choice)) in available_choices.iter().enumerate() {
            println!("{}. {}", display_idx + 1, choice.label);
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

        let scene = runtime.current_scene();
        let selected_choice = &scene.choices[choice];
        logger.log_choice(choice, &selected_choice.label);
        logger.log_effects(&selected_choice.effects);

        if let Err(e) = runtime.choose(choice) {
            eprintln!("Error: {}", e);
            break;
        }

        logger.log_scene(runtime.current_scene_id());
        logger.log_state(&runtime);
        clear_screen();
    }
}

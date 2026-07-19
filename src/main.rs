use std::env;

mod quotes;
mod creatures;
mod cowsay;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut quote = crate::quotes::generate_quote();
    let mut creature_id = "-b";
    let creatures = crate::creatures::get_creatures();
    
    if args.len() > 1 {
        if args[1] == "-v" || args[1] == "--version" {
            println!("minsay v{}", env!("CARGO_PKG_VERSION"));
            return;
        } if args[1] == "-h" || args[1] == "--help" {
            println!("usage: minsay [OPTIONS] <message>

options:
      -h, --help        display this help and exit
      -v, --version     display version number and exit

creatures: ");
            let mut keys: Vec<&str> = creatures.keys().copied().collect();
            keys.sort();
            for key in keys {
                if let Some(creature) = creatures.get(key) {
                    println!("      {}\t\t{}", key, creature.desc);
                }
            }
            return;
        } else if creatures.contains_key(args[1].as_str()) {
            creature_id = &args[1];
            if args.len() > 2 {
                quote = args[2..].join(" ");
            }
        } else {
            quote = args[1..].join(" ");
        }
    }
    
    let creature = creatures.get(creature_id).unwrap_or(&creatures["-b"]);
    crate::cowsay::cowsay(&quote, creature);
}

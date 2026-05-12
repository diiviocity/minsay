use rand::prelude::*;
use std::env;

fn cowsay(text: &str) {
    let max_line_length = 40;
    let mut words: Vec<&str> = text.split_whitespace().collect();
    let mut lines: Vec<String> = Vec::new();

    while !words.is_empty() {

        let mut current_line = String::new();
        while !words.is_empty() &&
                                current_line.len() + words[0].len() + 
                                    if current_line.is_empty() { 0 } else { 1 }
                                                                                <= max_line_length {
            
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(words[0]);
            words.remove(0);
        }

        lines.push(current_line);
    }

    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    println!("╭{}╮", "─".repeat(max_width + 2));
    for line in lines.iter() {
        println!("│ {}{} │", line, " ".repeat(max_width - line.len()));
    }
    println!("╰{}╯", "─".repeat(max_width + 2));
    println!("        \\   ^__^");
    println!("         \\  (oo)\\_______");
    println!("            (__)\\       )\\/\\");
    println!("                ||----w |");
    println!("                ||     ||");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let quote = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        let mut rng = rand::rng();
        [
            "if u ever feel lonely don't forget about the algorithms tracking ur every move... someone cares!",
            "u should use reiserfs in honour of its creator!",
            "the 76 in System76 stands for 76% of background cpu usage",
            "why are you using a POSIX shell descendant in 2026? use a proper interpreter... like SBCL... smh",
            "your window manager has too much, you should really consider writing your own one in c for your specific use cases",
            "why search for a distro that uses the software that doesn't suck when you can just choose the software yourself and not be an idiot?...",
            "tech bros are a myth of the ancient skoofs",
            "i paid for the entire processor to compile the linux kernel with the entire processor, bloat slows us down",
            "why is swap on by default? are redhat trying to increase the ssd sales?",
            "all of the mainstream init systems are too bloated, you should use BSD-style init scripts or just kill yourself",
            "lol imagine having age verification in your pid 1",
            "it seems that our poor telegram desktop client is allergic to not crashing",
            "x sucks, wayland sucks, dbus sucks, you suck, everything sucks!",
            "in the Unix world any piece of software that requires the user to exit the TTY sucks",
            "when you middle-click, you're pasting fascism!",
            "whether you believe it or not security by obscurity is a real and working principle",
            "I don't use Arch btw",
            "i love using software that is memory safe, optimized, blazingly fast and takes 15 minutes to compile",
            "your web browser has more lines of code than the linux kernel.", 
            "we need to add age verification to rust in order to make it safer!",
            "touch grass? i aliased that to rm -rf /",
            "looks like someone has more commits in their dotfiles repo than in their actual projects...",
            "uhm actually it's not GNU/Linux but rather systemd/GNU+Linux",
            "the only places for icons is in a church, a burning church at that",
            "most software today is like an egyptian pyramid with millions of bricks piled on top of each other, with no structural integrity, but just done by brute force and thousands of slaves",
            "you can't trust code that you did not totally create yourself!",            
            "simplicity is the ultimate sophistication",
            "software sucks because users demand it to. murder the users, avoid becoming part of the problem.",
            "languages that try to disallow idiocy become themselves idiotic",
            "if you have any trouble sounding condescending, find a Unix user to show you how it's done",
            "2 of the most famous products of Berkeley are LSD and Unix and I don't think that is a coincidence",
            "Unix is user-friendly, it's just very selective about who its friends are",
            "RMS is to Unix, like Hitler to Nietzsche",
            "In the early days Unix programs had two design rules: 1. It didn't have to be good or correct. 2. It had to be small. As hardware became more powerful, rule 2 has been relaxed.",
            "i don't have \"impostor syndrome\", i am just capable of correct assessing my own incompetence",
            "Rust succumbed to being a fetishised, slow, and unpleasant programming language used mostly by degenerates",
            "electron applications are what happens when the developer is pretending to not hate themselves but secretly wants others to hate them",
            "the internet of things is the modern equivalent of pre-industrialization plumbing systems",
            "Go is just a worse version of Rust: an attempt at a C++ rewrite, but even more naive",
            "bloat is in the eye of the beholder and i behold everything as bloated",
            "docker was created for the wrongdoers who are willing to sacrifice native performance for compiling a docker image for 4 hours",
            "life is too short to run proprietary software",
            "All graphics on Unix are bad, that's because modern Unix is barbaric technology",
            "fonts are a distribution problem disguised as a rendering problem",
            "if a program can't rewrite its own code, what good is it?",
            "You know the real trouble with Unix? The real trouble is that it became so popular. It wasn't meant to be popular.",
            "there are so many bastards to choose from why not kill them all?",
            "tomorrow brings a new distro and it's better than the last you know: another million bits have changed!",
            "The world is Unix, Unix is the world, laboring ceaselessly for the salvation of all sentient beings",
            "firefox is always doing something, even if it's just calculating the opportune moment to crash inexplicably",
            "i don't have a \"superiority complex\", i just understand that i'm superior",
            "Bill Gates says: \"Abstraction layers are like condoms: you should wear at least three!... Otherwise you're a terrorist!\"",
            "an idiot admires complexity, a genius admires simplicity",
            "Software is never finished, only abandoned or rewritten in Rust for some fucking reason",
            "All software sucks."
        ].choose(&mut rng).unwrap().to_string()
    };
    
    cowsay(&quote); 
}

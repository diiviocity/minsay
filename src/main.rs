use rand::prelude::*;
use std::env;

fn cowsay(text: &str) {
    const MAX_LINE_WIDTH: usize = 40;
    let mut words: Vec<&str> = text.split_whitespace().collect();
    let mut lines: Vec<String> = Vec::new();

    while !words.is_empty() {

        let mut current_line = String::new();
        
        while !words.is_empty() && current_line.chars().count() + words[0].chars().count() <= MAX_LINE_WIDTH {
                if !current_line.is_empty() { current_line.push(' '); }
                current_line.push_str(words[0]);
                words.remove(0);
        }
        
        lines.push(current_line);
    }

    let max_width = lines.iter().map( |line|  line.chars().count() )
                                .max().unwrap_or(0);

    println!("╭{}╮", "─".repeat(2 + max_width));
    for line in lines.iter() {
        let padding = max_width - line.chars().count();
        println!("│ {}{} │", line, " ".repeat(padding));
    }
    println!("╰{}╯", "─".repeat(2 + max_width));

    println!("        \\   ^__^");
    println!("         \\  (oo)\\_______");
    println!("            (__)\\       )\\/\\");
    println!("                ||----w |");
    println!("                ||     ||");
}


fn generate_quote() -> String {
    let mut rng = rand::rng();
    let quotes = [
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
        "languages that try to disallow idiocy become idiotic themselves",
        "if you have any trouble sounding condescending, find a Unix user to show you how it's done",
        "2 of the most famous products of Berkeley are LSD and Unix and I don't think that is a coincidence",
        "unix is user-friendly, it's just very selective about who its friends are",
        "rms is to unix, like hitler to nietzsche",
        "In the early days Unix programs had two design rules: 1. It didn't have to be good or correct. 2. It had to be small. As hardware became more powerful, rule 2 has been relaxed.",
        "i don't have \"impostor syndrome\", i am just capable of correctly assessing my own incompetence",
        "electron applications are what happens when the developer is pretending to not hate themselves but secretly wants others to hate them",
        "the internet of things is the modern equivalent of pre-industrialization plumbing systems",
        "go is just a worse version of rust: an attempt at a c++ rewrite, but even more naive",
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
        "software is never finished, only abandoned or rewritten in rust for some fucking reason",
        "All software sucks.",
        "using a GUI is like being a sub in BDSM: your every action is constrained...",
        "Ruby on Rails, the PHP of a new generation",
        "software security placebos need to keep getting more complex to keep working",
        "apple owners do it with mice!",
        "when in doubt, use brute force",
        "real linux users don't use a wallpaper",
        "you built a frame-buffer driver for your kernel? bloat.",
        "I like offending people because I think people who get offended should be offended",
        "Lennart \"You will own nothing\" Poettering",
        "minimalism is to autism as having friends is to everyone else",
        "it's cute how people who know nothing pretend they know things",
        "sandboxing isn't a silver bullet, those famously don't exist in security.",
        "Someone should make a distro called Arch BTW made for posers, coming with a pre-riced Hyprland and absolutely no bloat (aka nothing except for a flashy translucent terminal and fastfetch)",
        "Set up a script which will randomly cycle through all WMs/DEs that you have installed per restart or log in!",
        "XDG knows better than users what dirs they want, and WILL mkdir them. Then you MAY delete them.",
        "guys if you don't like malware, just compile every piece of software yourself, after you review every line of source code (and fix a few bugs while you're at it).",
        "lutris: the installer that installs installers that install installers (written in python btw)",
        "why are you calling foss devs dumb when you haven't gone into the open source code and fixed it yourself?...",
        "Very soon canonical will decide that all GTK users must stop drinking coffee with caffeine...",
        "hyprland went from changing syntax every few minutes to now changing programming languages every few minutes",
        "i already switched to linux, how much time do i have before a boykisser appears on my pfp?",
        "so... is this \"cachyos\" in the room with us right now?",
        "OPENSUSE MENTIONED!!!",
        "i'd rather send letters by mail than use discord, we need an irc client that appeals to stupid people!",
        "don't forget to run your daily \"pacman -Syyu\" at 3 am!",
        "HIV is no longer a terminal disease?.. Does it have a GUI now?!",
        "you use snaps? for what? for the copilot desktop?",
        "You should write an efi-executable static fastfetch in no-std rust",
        "Ульяновск.BSD btw",
        "Does X comply with the Unix philosophy?",
        "I'd just like to interject for a moment...",
        "systemd-free? what about udev and dbus?",
        "CachyOS Update: Reboot recommended! Reboot is recommended due to the upgrade of core system package(s).",
        "how many times I have to tell you guys, usability isn't what counts, that's consumer mindset. freedom matters. unfortunately, the redhat cronies try day and night to take it away from us. and your \"I just want an OS that works\"-ass mentality isn't helping",
        "issue ignored, works on my machine",
        "when you go to a linux girl's house but she uses debian with snap:",
        "can't spell hatred without redhat",
        "alias syutdown=\"pacman -Syu && poweroff\"",
        "90% of distrohoppers quit right before finding the perfect distro!",
        "systemdeez nuts",
        "remember kids, install linux-headers beforehand",
        "You can't call yourself a Linux user until you install a custom OS, with a package format used by 3 people or less, directly to a spinning platter with a nail and carefully timed taps with a magnet",
        "No job. No partner. Just a Nix config and a rice.",
        "Please don't theme our apps!",
        "*nix users try not to mention the windows challenge",
        "\"Linux desktop\" is a fake term likely invented by Red Hat or XDG to appeal to the non technically inclined. Every distribution able to run X11 is capable of being a \"desktop\". The term is a dogwhistle for pushing GUI tools and idiot-appealing features.",
    ];
    return quotes.choose(&mut rng).unwrap().to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let quote = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        generate_quote()
    };
    
    cowsay(&quote); 
}

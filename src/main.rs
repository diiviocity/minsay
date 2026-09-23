use std::{borrow::Cow, env};

use core::fmt;

mod quotes;
mod creatures;
mod cowsay;

use creatures::{Creature, CREATURES};

enum MsgType {
    None,
    NotAFlag(String),
    Flag(Creature),
    Help,
    Version,
}

struct Quote(pub Cow<'static, str>);

struct HelpMsg;

impl MsgType {
    fn match_mb_flags(mb_flags: Option<String>) -> Self {
        let mb_flags = match mb_flags {
            Some(v) => v, None => return Self::None
        };

        match &*mb_flags {
            "-v" => Self::Version,
            "--version" => Self::Version,
            "-h" => Self::Help,
            "--help" => Self::Help,
            // Return back the string if it couldn't be parsed as a flag
            _ => CREATURES.get(&mb_flags)
                .map(|&c| Self::Flag(c))
                .unwrap_or_else(|| Self::NotAFlag(mb_flags))
        } 
    }
}

impl Quote {
    fn merge_or_random(first_word: Option<String>, other_words: impl Iterator<Item = String>) -> Self {
        // Reuse first word buffer for pushing other words if it exists,
        // otherwise allocate a random String from QUOTES
        let mut buf = match first_word {
            None => return Self(Cow::Borrowed(quotes::generate_quote())),
            Some(s) => s
        };
        for word in other_words {
            buf.push_str(&word);
        }
        Self(Cow::Owned(buf))
    }
}


impl fmt::Display for HelpMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "usage: minsay [OPTIONS] <message>")?;
       
        // Render creatures
        for idx in 0..CREATURES.len() {
            let key = CREATURES.keys()[idx];
            let desc = CREATURES.arr[idx].desc;
            writeln!(f, "      {}\t\t{}", key, desc)?;
        }
        
        Ok(())
    }
}


fn main() {
    let mut args = env::args();
    args.next();

    // Determine message type and parse creature from flag or default if needed
    let (creature, first_word_of_quote) = match MsgType::match_mb_flags(args.next()) {
        MsgType::Help => {
            println!("{}", HelpMsg);
            return;
        }
        MsgType::Version => {
            println!("minsay v{}", env!("CARGO_PKG_VERSION"));
            return
        }
        MsgType::Flag(creature) => (creature, args.next()),
        MsgType::None => (CREATURES.cat(), args.next()),
        MsgType::NotAFlag(s) => (CREATURES.cat(), Some(s)),
    };
   
    let quote = Quote::merge_or_random(first_word_of_quote, args);
    crate::cowsay::cowsay(&quote.0, &creature);
}

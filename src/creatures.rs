#[derive(Clone, Copy)]
pub struct Creature {
	pub art: &'static str,
	pub desc: &'static str,
}

pub struct CreatureTable {
    pub arr: [Creature; Self::LEN]
}

impl CreatureTable {
    pub const LEN: usize = 11;
    pub const CAT: usize = 0;
    /// Table keys sorted in lexicographical and Creature index order
    pub const KEYS: [&'static str; Self::LEN] = [
        "-b", "-bd", "-bs", "-c", "-cb", "-cd", "-cg", "-cs", "-ct", "-cw", "-cy",
    ];
    pub const fn new() -> Self {
        let arr = [
            Creature {
                art: "\\    /\\       /\\
 \\  /  \\ __  /  \\
    |   \\) \\/   |
   _\\ ___   ___ /_
   \\_ ( ●   ● ) _/
    /_ ~  -  ~ _\\
      \\_  —  _/
       >     \\
       |     |",
                desc: "cat"
            },
            Creature {
                art: "\\    /\\       /\\
 \\  /  \\ __  /  \\
    |   \\) \\/   |
   _\\ ___   ___ /_
   \\_ ( X   X ) _/
    /_ ~  -  ~ _\\
      \\_  —  _/
       >     \\
       |     |",
                desc: "cat dead"
            },
            Creature {
                art: "\\    /\\       /\\
 \\  /  \\ __  /  \\
    |   \\) \\/   |
   _\\ ___   ___ /_
   \\_ ( ●   ● ) _/
    /_ ~  -  ~ _\\
      \\_  w  _/
       >     \\
       |     |",
                desc: "cat smiling"
            },
            Creature {
                art: "\\   ^__^
 \\  (oo)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
                desc: "cow"
            },
            Creature {
                art: "\\   ^__^
 \\  (==)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
                desc: "cow borg"
            },
            Creature {
                art: "\\   ^__^
 \\  (xx)\\_______
    (__)\\       )\\/\\
     U  ||----w |
        ||     ||",
                desc: "cow dead"
            },
            Creature {
                art: "\\   ^__^
 \\  ($$)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
                desc: "cow greedy"
            },
            Creature {
                art: "\\   ^__^
 \\  (**)\\_______
    (__)\\       )\\/\\
     U  ||----w |
        ||     ||",
                desc: "cow sleepy"
            },
            Creature {
                art: "\\   ^__^
 \\  (--)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
                desc: "cow tired"
            },
            Creature {
                art: "\\   ^__^
 \\  (OO)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
                desc: "cow wired"
            },
            Creature {
                art: "\\   ^__^
 \\  (..)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
                desc: "cow young"
            },
        ];
        Self { arr }
         
    }
    pub fn get(&self, flags: &str) -> Option<&Creature> {
        let idx = Self::KEYS.iter().position(|s| *s == flags)?;
        self.arr.get(idx) 
    }
    pub const fn keys(&self) -> &[&'static str; 11] {
        &Self::KEYS
    }
    pub const fn len(&self) -> usize {
        Self::LEN
    }
    pub const fn cat(&self) -> Creature {
        self.arr[Self::CAT]
    }
}

pub const CREATURES: CreatureTable = CreatureTable::new();


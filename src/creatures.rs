use std::collections::HashMap;

pub struct Creature {
	pub art: &'static str,
	pub desc: &'static str,
}

pub fn get_creatures() -> HashMap<&'static str, Creature> {
	HashMap::from([
		("-b",
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
			desc: "cat"}),
		("-bs",
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
			desc: "cat smiling"}),
		("-bd",
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
			desc: "cat dead"}),
		("-c",
		 Creature {
			art: "\\   ^__^
 \\  (oo)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
			desc: "cow"}),
		("-cb",
		 Creature {
			art: "\\   ^__^
 \\  (==)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
			desc: "cow borg"}),
		("-cd",
		 Creature {
			art: "\\   ^__^
 \\  (xx)\\_______
    (__)\\       )\\/\\
     U  ||----w |
        ||     ||",
			desc: "cow dead"}),
		("-cg",
		 Creature {
			art: "\\   ^__^
 \\  ($$)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
			desc: "cow greedy"}),
		("-cs",
		 Creature {
			art: "\\   ^__^
 \\  (**)\\_______
    (__)\\       )\\/\\
     U  ||----w |
        ||     ||",
			desc: "cow sleepy"}),
		("-ct",
		 Creature {
			art: "\\   ^__^
 \\  (--)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
			desc: "cow tired"}),
		("-cw",
		 Creature {
			art: "\\   ^__^
 \\  (OO)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
			desc: "cow wired"}),
		("-cy",
		 Creature {
			art: "\\   ^__^
 \\  (..)\\_______
    (__)\\       )\\/\\
        ||----w |
        ||     ||",
			desc: "cow young"}),
	])
}

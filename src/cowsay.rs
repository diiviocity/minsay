use meowstring::MeowString;

const MAX_LINE_WIDTH: usize = 40;
const EXPECTED_MAX_LINES: usize = 32;

pub fn cowsay(text: &str, creature: &crate::creatures::Creature) {
	let mut lines: Vec<MeowString<MAX_LINE_WIDTH>> = Vec::with_capacity(EXPECTED_MAX_LINES);

	for inline in text.split('\n') {
        let mut line = MeowString::<MAX_LINE_WIDTH>::new();
        
        for word in inline.split_whitespace() {
            if line.try_push_str(word).is_err() || line.try_push_str(" ").is_err() {
                lines.push(line.clone());
                line.clear(); 
            }
        }
        lines.push(line.clone())
	}

	let max_width = lines.iter().map( |line|  line.chars().count() )
	.max().unwrap_or(0);

	println!("╭{}╮", "─".repeat(2 + max_width));
	for line in lines.iter() {
		let padding = max_width - line.chars().count();
		println!("│ {}{} │", line, " ".repeat(padding));
	}
	println!("╰{}╯", "─".repeat(2 + max_width));

	let width_center = ((max_width) / 2).saturating_sub(6) + 4;
	for line in creature.art.lines() {
		println!("{}{}", " ".repeat(width_center), line);
	}
}

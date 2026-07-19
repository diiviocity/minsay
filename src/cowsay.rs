const MAX_LINE_WIDTH: usize = 40;

pub fn cowsay(text: &str, creature: &crate::creatures::Creature) {
	let mut lines: Vec<String> = Vec::new();

	for inline in text.split('\n') {
		let mut words: Vec<&str> = inline.split_whitespace().collect();

		while !words.is_empty() {
			let mut current_line = String::new();

			while !words.is_empty() && current_line.chars().count() + words[0].chars().count() <= MAX_LINE_WIDTH {
				if !current_line.is_empty() { current_line.push(' '); }
				current_line.push_str(words[0]);
				words.remove(0);
			}

			lines.push(current_line);
		}
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

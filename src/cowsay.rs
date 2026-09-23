use meowstring::MeowString;
use meowvec::MeowVec;

const MAX_LINE_WIDTH: usize = 40;
const MAX_LINES: usize = 64;

fn split_into_bounded_lines(text: &str) -> MeowVec<MeowString<MAX_LINE_WIDTH>, MAX_LINES> {
	let mut lines = MeowVec::new();
    let mut line = MeowString::<MAX_LINE_WIDTH>::new();
	
    for inline in text.split('\n') {
        for word in inline.split_whitespace() {
            if line.try_push_str(word).is_err() || line.try_push_str(" ").is_err() {
                lines.push(line.clone());
                line.clear(); 
            }
        }
        // Remove trailing b' '
        unsafe { line.as_bytes_mut().pop(); }
        lines.push(line.clone());
        line.clear();
	}

    lines
}

pub fn cowsay(text: &str, creature: &crate::creatures::Creature) {
    let lines = split_into_bounded_lines(text);
	
    let max_width = lines.iter().map( |line|  line.chars().count() )
	.max().unwrap_or(0);
   
    // these bitches are 3 bytes long
	println!("╭{}╮", MeowString::<{ (MAX_LINE_WIDTH + 2) * 3}>::repeat("─", 2 + max_width));
	for line in lines.iter() {
		let padding = max_width - line.chars().count();
		println!("│ {}{} │", line, MeowString::<MAX_LINE_WIDTH>::repeat(" ", padding));
	}
	println!("╰{}╯", MeowString::<{ (MAX_LINE_WIDTH + 2) * 3}>::repeat("─", 2 + max_width));

	let width_center = (max_width / 2).saturating_sub(2);
	for line in creature.art.lines() {
		println!("{}{}", MeowString::<{ (MAX_LINE_WIDTH / 2).saturating_sub(2) }>::repeat(" ", width_center), line);
	}
}

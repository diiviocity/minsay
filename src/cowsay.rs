use std::io;

use meowstring::MeowString;
use meowvec::MeowVec;

const MAX_LINE_WIDTH: usize = 40;
const MAX_LINES: usize = 32;

/// # Safety
///     The caller must ensure the text, after bounding, spans less than MAX_LINES lines.
///     Violating that invariant causes undefined behavior (hopefully a segfault)
unsafe fn split_into_bounded_lines(text: &str) -> MeowVec<MeowString<MAX_LINE_WIDTH>, MAX_LINES> {
	let mut lines = MeowVec::new();
    let mut line = MeowString::<MAX_LINE_WIDTH>::new();
	
    for inline in text.split('\n') {
        for word in inline.split_whitespace()  {
            if line.try_push_str(word).is_err() {
                //@MauntiCat
                // Causes undefined behavior when the bounded text spans >MAX_LINES lines :3
                unsafe { lines.push_unchecked(line.clone()); }
                line.clear();
                line.push_str(word);
            }
            let _ = line.try_push_str(" ");
        }
        //@MauntiCat
        unsafe {
            // Remove trailing b' '
            line.as_bytes_mut().pop();
            // Causes undefined behavior when the bounded text spans >MAX_LINES lines :3
            lines.push_unchecked(line.clone());
        }
        line.clear();
	}

    lines
}

/// # Safety
///     The caller must ensure the quote is not bloated.
///     Violating that invariant causes undefined behavior
pub unsafe fn cowsay(text: &str, creature: &crate::creatures::Creature) {
    let lines = unsafe { split_into_bounded_lines(text) };
	
    let max_width = lines.iter().map( |line|  line.chars().count() )
	.max().unwrap_or(0);
  
    // lock stdout beforehand for less overhead
    let lock = io::stdout().lock();
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
    drop(lock)
}

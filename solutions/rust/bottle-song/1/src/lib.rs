const COUNTS: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| create_verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn create_verse(bottles: u32) -> String {
    let current = bottles;
    let next = current - 1;
    format!(
        "{bottles} green bottle{s1} hanging on the wall,\n\
        {bottles} green bottle{s1} hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {next} green bottle{s2} hanging on the wall.",
        bottles = COUNTS[current as usize],
        next = COUNTS[next as usize].to_lowercase(),
        s1 = trailing_s(current),
        s2 = trailing_s(next)
    )
}

fn trailing_s(current_index: u32) -> &'static str {
    match current_index {
        1 => "",
        _ => "s",
    }
}

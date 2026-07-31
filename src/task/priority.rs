const PRIORITIES: [&'static str; 4] = ["None", "Low", "Medium", "High"];

pub const fn priority_text(priority: u8) -> &'static str {
    PRIORITIES[priority as usize]
}

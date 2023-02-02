include!(concat!(env!("OUT_DIR"), "/include"));

/// Returns UCD properties in JSON
pub fn get(char: char) -> &'static str {
    UCD_DATA[char as usize]
}

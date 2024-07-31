use rand::seq::SliceRandom;

use crate::mappings::MAPPING;

pub fn translate(message: &str) -> String {
    let mut translated = String::from("");
    for (i, c) in message.chars().enumerate() {
        let candidates = MAPPING.get(&c);
        if candidates.is_none() || i % 2 == 1 {
            translated.push(c);
        } else {
            let elected = candidates.unwrap().choose(&mut rand::thread_rng());
            match elected {
                None => {
                    translated.push(c);
                }
                Some(glyph) => {
                    translated.push_str(glyph);
                }
            }
        }
    }
    return translated;
}

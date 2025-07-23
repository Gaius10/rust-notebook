
pub fn say(s: &str) -> String {
    let mut text: String = String::new();

    for word in s.split_whitespace() {
        text.push_str(&(translate(word) + " "));
    }

    text.trim().to_string()
}

fn translate(s: &str) -> String {
    let lc = get_last_char(s);
    let mut c = get_first_char(s);
    let mut not_c = &s[1..s.len()];

    if is_vowel(c) {
        c = 'h';
        not_c = &s[0..s.len()];
    }

    if lc == '.' {
        not_c = &not_c[0..not_c.len() - 1];        
    }

    let mut word = format!("{not_c}-{c}ay");

    if lc == '.' {
        word = format!("{word}.");
    }

    if c.is_uppercase() {
        word = capitalize(&word);
    }

    word
}

fn get_first_char(s: &str) -> char {
    s.chars().nth(0).unwrap()
}

fn get_last_char(s: &str) -> char {
    if let Some(c) = s.chars().nth(s.len() - 1) {
        return c;
    };

    ' '
}

fn is_vowel(c: char) -> bool {
    ['a', 'e', 'i', 'o', 'u'].contains(
        &c.to_lowercase().nth(0).unwrap()
    )
}

fn capitalize(s: &str) -> String {
    let s = s.to_lowercase();
    let mut v: Vec<char> = s.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}





pub mod indexing;

pub fn piglatin(string: &String) -> Option<String> {
    if string.contains(" ") {
        return None;
    }
    let mut new_string = String::from(string.clone());
    let t = string.get(0..1);
    if let Some(s) = t {
        if vec!["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"].contains(&s) {
            new_string.push_str("-hay");
        } else {
            let q = new_string.remove(0);
            new_string.push_str("-");
            new_string.push_str(&(q.to_lowercase().to_string()));
            new_string.push_str("ay");
        }
        Some(new_string)
    } else {
        None
    }
}
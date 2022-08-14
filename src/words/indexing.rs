// Returns a slice to the i'th word in s. If there is no i'th word, returns the last
pub fn find_word_i(s: &str, i: u32) -> Option<&str> {
    let mut count = i;
    let mut prevs = 0;
    let mut ret = None;

    for (j, item) in s.char_indices() {
        if item == ' ' {
            if count == 0 {
                ret = Some(&s[prevs..j]);
            } else {
                prevs = j + 1;
                count -= 1;
            }
        }
    }
    ret
}

// Keep this private for poops and giggles
fn _first_word(s: &str) -> &str {
    // The return is a slice
    let bytes = s.as_bytes(); // Gives a more c style string, not for UTF 8

    // Iter goes over and gives item, enumerate gives the i
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // remember byte literal syntax
            return &s[0..i];
        }
    }
    return &s[..]; // Default case, no space return entire word
}

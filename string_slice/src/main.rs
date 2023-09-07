//* write a function that takes a string of words separated by spaces and returns the first word 
//* it finds in that string. If the function doesn’t find a space in the string, 
//* the whole string must be one word, so the entire string should be returned

fn main() {
    let mut s: String = String::from("hello world");
    let word = first_word(&s);
    
    print!("{}", word);

    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i] }
    }

    &s[..]
}


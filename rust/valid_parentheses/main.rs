use std::collections::HashMap;

fn main() {
    let word: String = String::from("()[]{}");
    let is_valid = validate_parentheses(word);
    println!("A string é válida? {}", is_valid);

    let word2: String = String::from("([)]");
    let is_valid2 = validate_parentheses(word2);
    println!("A string 2 é válida? {}", is_valid2);
}

fn validate_parentheses(word: String) -> bool {
    let mut open_brackets = HashMap::new();

    for (i, c) in word.chars().enumerate() {
        if c == '(' || c == '{' || c == '[' {
            open_brackets.insert(i, c);
        }
        
        if c == ')' || c == '}' || c == ']' {
            let last_open_key = open_brackets.keys().max().cloned();
            if let Some(key) = last_open_key {
                let open_char = open_brackets[&key];
            
                let is_match = (open_char == '(' && c == ')') ||
                               (open_char == '{' && c == '}') ||
                               (open_char == '[' && c == ']');

                if is_match {
                    open_brackets.remove(&key);
                } else {               
                    return false;
                }
            } else {
                return false;
            }
        }
    }





    open_brackets.is_empty()
}
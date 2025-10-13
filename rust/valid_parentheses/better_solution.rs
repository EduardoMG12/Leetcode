fn main() {
    let word: String = String::from("()[]{}");
    let is_valid = validate_parentheses(word);
    println!("A string '{}' é válida? {}", "()[]{}", is_valid)

    let word2: String = String::from("([)]");
    let is_valid2 = validate_parentheses(word2);
    println!("A string '{}' é válida? {}", "([)]", is_valid2)

    let word3: String = String::from("{[]}");
    let is_valid3 = validate_parentheses(word3);
    println!("A string '{}' é válida? {}", "{[]}", is_valid3)

    let word4: String = String::from("]");
    let is_valid4 = validate_parentheses(word4);
    println!("A string '{}' é válida? {}", "]", is_valid4)
}

fn validate_parentheses(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else if c == ')' || c == ']' || c == '}' {    
            match stack.last() {
        
                Some(&top) => {
            
                    if (top == '(' && c == ')') ||
                       (top == '{' && c == '}') ||
                       (top == '[' && c == ']') {
                        stack.pop()
                    } else {
                        return false
                    }
                },
        
                None => return false,
            }
        }
    }

    stack.is_empty()
}
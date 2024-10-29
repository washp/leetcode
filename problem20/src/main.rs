pub fn is_valid(s: String) -> bool {
    let mut char_stack: Vec<char> = vec![];
    for char in s.chars() {
        match char {
            '(' => char_stack.push(')'),
            '[' => char_stack.push(']'),
            '{' => char_stack.push('}'),
            ')' | ']' | '}' if char_stack.pop() != Some(char) => return false,
            _ => (),
        }
    }
    char_stack.is_empty()
}

fn main() {
    println!("No implementation, sorry!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = is_valid("()".to_string());
        assert!(result);
    }
    #[test]
    fn test_2() {
        let result = is_valid("()[]{}".to_string());
        assert!(result);
    }
    #[test]
    fn test_3() {
        let result = is_valid("(]".to_string());
        assert!(!result);
    }
    #[test]
    fn test_4() {
        let result = is_valid("([])".to_string());
        assert!(result);
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' | '}' | ']' => match (c, stack.pop()) {
                (')', Some('(')) => {}, 
                ('}', Some('{')) => {},
                (']', Some('[')) => {},
                _ => return false,
            }, 
            _ => {}
        }
    }
    stack.is_empty()
}

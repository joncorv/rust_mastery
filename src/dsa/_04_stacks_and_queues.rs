//! Stacks & Queues. Notes live in book/src/patterns/stacks_and_queues.md.

// ANCHOR: valid_parentheses
/// 20. Valid Parentheses: push the EXPECTED closer, compare on pop.
pub fn valid_parentheses(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
// ANCHOR_END: valid_parentheses

// ANCHOR: next_greater
/// Monotonic stack: for each i, the first value to its right that's bigger (-1 if none).
pub fn next_greater(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![-1; nums.len()];
    let mut stack: Vec<usize> = Vec::new(); // INDICES; their values are decreasing
    for (i, &x) in nums.iter().enumerate() {
        while let Some(&top) = stack.last()
            && nums[top] < x
        {
            result[top] = x; // x is the answer for everything smaller on the stack
            stack.pop();
        }
        stack.push(i);
    }
    result
}
// ANCHOR_END: next_greater

// ANCHOR: eval_rpn
/// 150. Evaluate Reverse Polish Notation: operands on a stack, operators pop two.
pub fn eval_rpn(tokens: &[&str]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for &t in tokens {
        match t {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().unwrap(); // pop order matters: b is the RIGHT operand
                let a = stack.pop().unwrap();
                stack.push(match t {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    _ => a / b,
                });
            }
            num => stack.push(num.parse().unwrap()),
        }
    }
    stack[0]
}
// ANCHOR_END: eval_rpn

#[test]
fn test_stacks_and_queues() {
    assert!(valid_parentheses("([]{})"));
    assert!(!valid_parentheses("(]"));
    assert!(!valid_parentheses("(("));
    assert!(!valid_parentheses(")"));
    assert_eq!(next_greater(&[2, 1, 2, 4, 3]), [4, 2, 4, -1, -1]);
    assert_eq!(eval_rpn(&["4", "13", "5", "/", "+"]), 6);
}

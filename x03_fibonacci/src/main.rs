use std::vec;

/// Returns nth element in fibonacci series
/// Index starts at zero
///
/// Time Complexity: O(2^n)
/// Space Complexity: O(1)
fn fib(n: u32) -> u128 {
    if n <= 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

/// Returns nth element in fibonacci series
/// Index starts at zero
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
fn fib2(n: u32) -> u128 {
    let mut stack = vec::Vec::new();

    for i in 0..=n {
        if i <= 1 {
            stack.push(1);
        } else {
            stack.push(stack[(i as usize) - 1] + stack[(i as usize) - 2]);
        }
    }

    match stack.pop() {
        Some(val) => {
            return val;
        },
        None => {
            println!("[!!] Stack is empty");
            return 0;
        }
    };
}

fn main() {
    print!("Using recursive approach: ");

    for n in 0..=20 {
        print!("{} ", fib(n));
    }

    print!("\nUsing iterative approach: ");

    for n in 0..=20 {
        print!("{} ", fib2(n));
    }

    println!("");
}
use std::io;

pub fn dc() {
		let mut stack: Vec<i32> = Vec::new();

		'dc: loop {
			let mut line = String::new();
			let mut number: i32;

			io::stdin().read_line(&mut line).expect("Failed to read line");
			for cmd in line.split_whitespace() {
				let c = cmd.chars().next().unwrap();
				if c.is_digit(10) {
					number = cmd.parse().expect("NaN");
					stack.push(number);
				}
				else  {
					match c {
						'+' => stack = add(stack),
						'-' => stack = substracte(stack),
						'*' => stack = multiply(stack),
						'/' => stack = divide(stack),
						'%' => stack = modular(stack),
						'P' => stack = pop_stack(stack),
						'p' => peek_stack(&stack),
						'f' => print_stack(&stack),
						'q' => break 'dc,
						_ => println!("unk")
					}
				}
			}
		}

	}

fn add (mut stack: Vec<i32>) -> Vec<i32> {
    if stack.len() >= 2 {
        let n1 : i32 = stack.pop().unwrap();
        let n2 : i32 = stack.pop().unwrap();
        stack.push(n2 + n1);
    }
    else {
        println!("dc: stack empty");
    }
    stack
}
fn substracte (mut stack: Vec<i32>) -> Vec<i32> {
    if stack.len() >= 2 {
        let n1 : i32 = stack.pop().unwrap();
        let n2 : i32 = stack.pop().unwrap();
        stack.push(n2 - n1);
    }
    else {
        println!("dc: stack empty");
    }
    stack
}
fn multiply (mut stack: Vec<i32>) -> Vec<i32> {
    if stack.len() >= 2 {
        let n1 : i32 = stack.pop().unwrap();
        let n2 : i32 = stack.pop().unwrap();
        stack.push(n2 * n1);
    }
    else {
        println!("dc: stack empty");
    }
    stack
}
fn divide (mut stack: Vec<i32>) -> Vec<i32> {
    if stack.len() >= 2 {
        let n1 : i32 = stack.pop().unwrap();
        let n2 : i32 = stack.pop().unwrap();
        stack.push(n2 + n1);
    }
    else {
        println!("dc: stack empty");
    }
    stack
}
fn modular (mut stack: Vec<i32>) -> Vec<i32> {
    if stack.len() >= 2 {
        let n1 : i32 = stack.pop().unwrap();
        let n2 : i32 = stack.pop().unwrap();
        stack.push(n2 + n1);
    }
    else {
        println!("dc: stack empty");
    }
    stack
}
fn pop_stack (mut stack: Vec<i32>) -> Vec<i32> {
	if stack.len() > 0 {
		peek_stack(&stack);
		stack.pop();
	}
	else {
		println!("dc: stack empty");
	}
    stack
}
fn peek_stack (stack: &Vec<i32>) {
	if stack.len() > 0 {
		println!("{}", stack.last().unwrap());
	}
	else {
        println!("dc: stack empty");
    }
}
fn print_stack (stack: &Vec<i32>) {
    let mut rev_stack = stack.clone();
    rev_stack.reverse();
    for num in rev_stack.iter() {
        println!("{}", num);
    }
}

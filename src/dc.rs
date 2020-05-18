use std::net::{TcpStream, Shutdown};
use std::io::prelude::*;

pub fn dc(line: String, mut stack: Vec<i32>, stream: &TcpStream) -> Vec<i32> {
	let mut number: i32;

	for cmd in line.split_whitespace() {
		let c = cmd.chars().next().unwrap();
		if c.is_digit(10) {
			number = cmd.parse().expect("NaN");
			stack.push(number);
		}
		else  {
			match c {
				'+' => stack = add(stack, stream),
				'-' => stack = substracte(stack, stream),
				'*' => stack = multiply(stack, stream),
				'/' => stack = divide(stack, stream),
				'%' => stack = modular(stack, stream),
				'P' => stack = pop_stack(stack, stream),
				'p' => peek_stack(&stack, stream),
				'f' => print_stack(&stack, stream),
				'q' => quit(stream),
				_ => continue,
			}
		}
	}
	stack
}

fn add (mut stack: Vec<i32>, mut stream: &TcpStream) -> Vec<i32> {
    if stack.len() >= 2 {
        let n1 : i32 = stack.pop().unwrap();
        let n2 : i32 = stack.pop().unwrap();
        stack.push(n2 + n1);
    }
    else {
			stream.write("dc: stack empty".as_bytes()).expect("Error");
    }
    stack
}

fn substracte (mut stack: Vec<i32>, mut stream: &TcpStream) -> Vec<i32> {
	if stack.len() >= 2 {
		let n1 : i32 = stack.pop().unwrap();
		let n2 : i32 = stack.pop().unwrap();
		stack.push(n2 - n1);
	}
	else {
		stream.write("dc: stack empty".as_bytes()).expect("Error");
	}
	stack
}

fn multiply (mut stack: Vec<i32>, mut stream: &TcpStream) -> Vec<i32> {
	if stack.len() >= 2 {
		let n1 : i32 = stack.pop().unwrap();
		let n2 : i32 = stack.pop().unwrap();
		stack.push(n2 * n1);
	}
	else {
		stream.write("dc: stack empty".as_bytes()).expect("Error");
	}
	stack
}

fn divide (mut stack: Vec<i32>, mut stream: &TcpStream) -> Vec<i32> {
	if stack.len() >= 2 {
		let n1 : i32 = stack.pop().unwrap();
		let n2 : i32 = stack.pop().unwrap();
		stack.push(n2 + n1);
	}
	else {
		stream.write("dc: stack empty".as_bytes()).expect("Error");
	}
	stack
}

fn modular (mut stack: Vec<i32>, mut stream: &TcpStream) -> Vec<i32> {
	if stack.len() >= 2 {
		let n1 : i32 = stack.pop().unwrap();
		let n2 : i32 = stack.pop().unwrap();
		stack.push(n2 + n1);
	}
	else {
		stream.write("dc: stack empty".as_bytes()).expect("Error");
	}
	stack
}

fn pop_stack (mut stack: Vec<i32>, mut stream: &TcpStream) -> Vec<i32> {
	if stack.len() > 0 {
		peek_stack(&stack, stream);
		stack.pop();
	}
	else {
		stream.write("dc: stack empty".as_bytes()).expect("Error");
	}
	stack
}

fn peek_stack (stack: &Vec<i32>, mut stream: &TcpStream) {
	if stack.len() > 0 {
		stream.write(format!("{}\n", stack.last().unwrap()).as_bytes()).expect("Error");
	}
	else {
		stream.write("dc: stack empty".as_bytes()).expect("Error");
	}
}

fn print_stack (stack: &Vec<i32>, mut stream: &TcpStream) {
	let mut rev_stack = stack.clone();
	rev_stack.reverse();
	for num in rev_stack.iter() {
		stream.write(format!("{}\n", num).as_bytes()).expect("Error");
	}
}

fn quit(stream: &TcpStream) {
	stream.shutdown(Shutdown::Both).expect("Error");
}
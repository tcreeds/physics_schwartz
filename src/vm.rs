
use parser;

use std::collections::HashMap;

use parser::*;

#[derive(Clone, Debug)]
enum Opcode {
	Push(f64),
	Load(usize),
	Add,
	Mul,
	Sub,
	Div,
	Call(String),
}
use self::Opcode::*;


#[derive(Debug)]
pub struct VM {
	instructions: Vec<Opcode>,
}
impl VM {
	pub fn compile(target: Expr, registers: &HashMap<&str, usize>) -> VM {
		VM {
			instructions: compile_expr(target, registers),
		}
	}
	pub fn run(&self, registers: &Vec<f64>) -> f64 {
		let mut stack: Vec<f64> = vec![];
		for op in self.instructions.iter().cloned() {
			match op {
				Push(num) => stack.push(num),
				Load(num) => stack.push(registers[num]),
				Add => { 
					let result = stack.pop().unwrap() +	stack.pop().unwrap();
					stack.push(result)
				},
				Sub => { 
					let result = stack.pop().unwrap() -	stack.pop().unwrap();
					stack.push(result)
				},
				Mul => { 
					let result = stack.pop().unwrap() *	stack.pop().unwrap();
					stack.push(result)
				},
				Div => { 
					let result = stack.pop().unwrap() /	stack.pop().unwrap();
					stack.push(result)
				},
				Call(func_name) => {
					match &func_name[..] {
						"sin" =>  { 
							let result = stack.pop().unwrap().sin();
							stack.push(result)
						},
						"cos" =>  { 
							let result = stack.pop().unwrap().cos();
							stack.push(result)
						},
						"sqrt" => {
							let result = stack.pop().unwrap().sqrt();
							stack.push(result)
						}
						_ => (),
					}
				}
			}
		};
		stack.pop().unwrap()
	}
}

fn compile_expr(target: Expr, registers: &HashMap<&str, usize>) -> Vec<Opcode> {
	match target {
		Expr::Number(val) => vec![Push(val)],
		Expr::Variable(name) => vec![Load(registers[&name[..]])],
		Expr::Call(func_name, args) => {
			let mut ret = vec![];
			for expr in args {
				ret.extend(compile_expr(expr, registers));
			}
			ret.push(Call(func_name));
			ret
		}
		Expr::Binary(lhs, op, rhs) => {	
			let mut ret = vec![];
			ret.extend(compile_expr(*rhs, registers));
			ret.extend(compile_expr(*lhs, registers));
			ret.push(match &op[..] {
				"+" => Add,
				"-" => Sub,
				"*" => Mul,
				"/" => Div,
				x => Call(x.to_string()),
			});
			ret
		} 
	}
}
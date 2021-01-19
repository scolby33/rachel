use itertools::Itertools;
use rayon::prelude::*;
use std::{env, fmt, iter};

// See sysexits(3)
static EX_USAGE: i32 = 64;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Element {
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(usize),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Add => "+".to_string(),
            Self::Subtract => "-".to_string(),
            Self::Multiply => "*".to_string(),
            Self::Divide => "/".to_string(),
            Self::Number(x) => x.to_string(),
        };
        write!(f, "{}", msg)
    }
}

const OPERATIONS: [Element; 4] = [
    Element::Add,
    Element::Subtract,
    Element::Multiply,
    Element::Divide,
];

fn compute(expression: &[&Element]) -> Option<usize> {
    let mut stack: Vec<usize> = Vec::with_capacity(6);
    for element in expression {
        let res: usize = match element {
            Element::Number(x) => *x,
            Element::Add => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                b.checked_add(a)?
            }
            Element::Subtract => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                b.checked_sub(a)?
            }
            Element::Multiply => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                b.checked_mul(a)?
            }
            Element::Divide => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let rem = b.checked_rem(a);
                match rem? {
                    0 => (),
                    _ => return None,
                };
                b.checked_div(a)?
            }
        };
        stack.push(res);
    }
    Some(*stack.last().expect("oops!"))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 7 {
        std::process::exit(EX_USAGE);
    }

    let numbers = vec![
        Element::Number(args[0].parse().expect("not a positive integer")),
        Element::Number(args[1].parse().expect("not a positive integer")),
        Element::Number(args[2].parse().expect("not a positive integer")),
        Element::Number(args[3].parse().expect("not a positive integer")),
        Element::Number(args[4].parse().expect("not a positive integer")),
        Element::Number(args[5].parse().expect("not a positive integer")),
    ];
    let target = Some(args[6].parse().expect("not a positive integer"));

    let res = iter::repeat(&OPERATIONS)
        .take(5)
        .multi_cartesian_product()
        .par_bridge()
        .flat_map(|operations| {
            let expression = numbers.iter().chain(operations);
            expression
                .clone()
                .permutations(11)
                .chain(expression.clone().permutations(9))
                .chain(expression.clone().permutations(7))
                .chain(expression.clone().permutations(5))
                .chain(expression.clone().permutations(3))
                .par_bridge()
        })
        .find_any(|perm| compute(&perm) == target);

    match res {
        Some(expression) => {
            let msg = expression
                .iter()
                .map(|element| element.to_string())
                .join(" ");
            println!("{}", msg);
        }
        None => println!("no solution!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert!(compute(&[&Element::Number(1), &Element::Number(2), &Element::Add]) == Some(3));
        assert!(compute(&[&Element::Number(2), &Element::Number(2), &Element::Divide,]) == Some(1));
        assert!(
            compute(&[&Element::Number(2), &Element::Number(4), &Element::Multiply,]) == Some(8)
        );
        assert!(
            compute(&[
                &Element::Number(7),
                &Element::Number(2),
                &Element::Number(3),
                &Element::Multiply,
                &Element::Subtract,
            ]) == Some(1)
        );
        assert!(
            compute(&[
                &Element::Number(7),
                &Element::Number(100),
                &Element::Number(25),
                &Element::Subtract,
                &Element::Multiply,
                &Element::Number(3),
                &Element::Number(10),
                &Element::Number(2),
                &Element::Add,
                &Element::Multiply,
                &Element::Add
            ]) == Some(561)
        );
    }
}

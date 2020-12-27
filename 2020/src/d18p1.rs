use nom::IResult;

enum Operator {
    Sum,
    Multiplication,
}

fn parse_math_expression(
    input: &str,
    acc: u64,
    pending_operation: Option<Operator>,
) -> IResult<&str, u64> {
    let take_o = input.get(0..1);

    match take_o {
        Some(c) => {
            let rest = &input[1..];
            match c {
                " " => parse_math_expression(rest, acc, pending_operation),
                "(" => {
                    let (r, pa) = parse_math_expression(rest, 0, Some(Operator::Sum)).unwrap();
                    match pending_operation {
                        Some(Operator::Multiplication) => parse_math_expression(r, acc * pa, None),
                        Some(Operator::Sum) => parse_math_expression(r, acc + pa, None),
                        None => unreachable!(),
                    }
                }
                ")" => Ok((rest, acc)),
                "*" => parse_math_expression(rest, acc, Some(Operator::Multiplication)),
                "+" => parse_math_expression(rest, acc, Some(Operator::Sum)),
                digit => {
                    let num = digit.parse::<u64>().expect("expect it to be a digit");
                    match pending_operation {
                        Some(Operator::Multiplication) => {
                            parse_math_expression(rest, acc * num, None)
                        }
                        Some(Operator::Sum) => parse_math_expression(rest, acc + num, None),
                        None => unreachable!(),
                    }
                }
            }
        }
        None => Ok((input, acc)),
    }
}

pub fn parse_math_expressions(input: &str) -> Vec<u64> {
    input
        .split('\n')
        // remove parenthesis
        //.map(|line| line.chars().filter(|c| *c != '(' && *c != ')').collect())
        .map(|line| {
            parse_math_expression(line, 0, Some(Operator::Sum))
                .unwrap()
                .1
        })
        .collect()
}

pub fn solve(input: &str) -> String {
    let numbers = parse_math_expressions(input);

    println!("{:?}", numbers);

    numbers.iter().sum::<u64>().to_string()
}

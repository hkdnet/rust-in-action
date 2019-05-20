fn main() {
    rpn::rpn();
}

mod rpn {
    pub fn rpn() {
        let expr = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
        let ans = solve(expr);

        debug_assert_eq!("26.2840", format!("{:.4}", ans));

        println!("ans = {:.4}", ans);
    }

    pub fn solve(expr: &str) -> f64 {
        let mut stack = Vec::new();
        for token in expr.split_whitespace() {
            if let Ok(f) = token.parse::<f64>() {
                stack.push(f);
            } else {
                match token {
                    "+" => apply2(&mut stack, |x, y| x + y),
                    "-" => apply2(&mut stack, |x, y| x - y),
                    "*" => apply2(&mut stack, |x, y| x * y),
                    "/" => apply2(&mut stack, |x, y| x / y),
                    _ => {
                        panic!("Unknown operator: {}", token)
                    }
                }
            }
        }
        let ans = stack.pop().expect("Stack underflow!");
        if !stack.is_empty() {
            panic!("some value remained!: [{}]", stack.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(","));
        }
        ans
    }

    fn apply2<F: Fn(f64, f64) -> f64>(stack: &mut Vec<f64>, f: F)
    {
        // Note: the first popped value is y, not x!
        if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
            let z = f(x, y);
            stack.push(z);
        } else {
            panic!("Stack underflow!");
        }
    }
}

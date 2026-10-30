pub enum MathError {
    DivisionByZero,
    NegativeLogarithm,
    NegativeSquareRoot,
}

pub type MathResult = Result<f64, MathError>;

fn div(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

fn sqrt(x: f64) -> MathResult {
    if x < 0.0f64 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn ln(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::NegativeLogarithm)
    } else {
        Ok(x.ln())
    }
}

fn op_(x: f64, y: f64) -> MathResult {
    // 如果 div 方法失败了，那么返回 MathError::DivisionByZero
    let ratio = div(x, y)?;
    // 如果 ln 方法失败了，那么返回 MathError:NegativeLogarithm
    let ln = ln(ratio)?;

    sqrt(ln)
}

pub fn op(x: f64, y: f64) {
    match op_(x, y) {
        Ok(val) => println!("{}", val),
        Err(why) => panic!(
            "{}",
            match why {
                MathError::NegativeLogarithm => "logarithm of negative number",
                MathError::DivisionByZero => "division by zero",
                MathError::NegativeSquareRoot => "square root of negative number",
            }
        ),
    }
}

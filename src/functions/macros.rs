
macro_rules! one_arg {
    ($name:ident) => {
        (stringify!($name).into(), Value::Null)
    };
    ($name:ident = $value:expr) => {{
        use valueexpression::value_expression;
        (stringify!($name).into(), value_expression($value).unwrap().1)
    }};
}

macro_rules! func {
    (( $($arg:ident $( = $value:expr )* ),* ), $body:expr) => {
        SassFunction {
            args: FormalArgs::new(vec![ $( one_arg!($arg $( = $value)* ) ),* ]),
            body: Box::new($body),
        }
    };
}
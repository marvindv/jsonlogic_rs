/// Expects a function typically used for JsonLogic operators in the scope this macro is called in:
///
/// ```ignore
/// fn compute(args: &[Expression], data: &Data) -> Value
/// ```
///
/// Calls the `compute` function with the given arguments wrapped in `Expression::Constant` and an
/// empty data object created with `Data::empty()`.
#[macro_export]
macro_rules! compute_const {
    ($($args:expr),*) => {{
        #[allow(unused_mut)]
        let mut args_vec = vec![];
        $(
            args_vec.push($args);
        )*

        let expressions: Vec<Expression> = args_vec
            .iter()
            .map(|arg: &Value| Expression::Constant(arg.clone()))
            .collect();
        compute(&expressions, &Data::empty())
    }}
}

/// Expects a function typically used for JsonLogic operators in the scope this macro is called in:
///
/// ```ignore
/// fn compute(args: &[Expression], data: &Data) -> Value
/// ```
///
/// Expects two arguments. First an slice containing `Value`'s and second a `Data` instance.
/// Calls the `compute` function with the given `Value` slice where every element is wrapped in
/// `Expression::Constant` and the given data instance.
#[macro_export]
macro_rules! compute_const_with_data {
    ($args:expr, $data:expr) => {{
        // Avoid "temporary value dropped while borrowed" errors.
        let args: &[Value] = $args;
        let expressions: Vec<Expression> = args
            .iter()
            .map(|arg| Expression::Constant(arg.clone()))
            .collect();
        compute(&expressions, $data)
    }};
}

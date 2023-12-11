pub mod math_functions {
    // 加法函数
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    //减法函数
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // 乘法函数
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // 除法函数
    pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            Err("Cannot divide by zero")
        } else {
            Ok(a / b)
        }
    }
}

pub mod math_Utils {
    pub fn square(value: i32) -> i32 {
        return value*value
    }
    pub fn applyToVec( vec: Vec<i32>, function: fn (i32) -> i32) -> Vec<i32> {
        return vec.iter().map(|elem: &i32| function(*elem)).collect()
    }

}
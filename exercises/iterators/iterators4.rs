// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // if num == 0 {
    //     1
    // } else {
    //     num * factorial(num - 1)
    // }

    // .fold 方法在 Rust 中是一种迭代器方法，它用于将一个初始值与迭代器中的所有元素进行累积操作
    (1..=num).fold(1, |accumulator, element| accumulator * element)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

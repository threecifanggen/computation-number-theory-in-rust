// 常见的序列

//斐波那契数列
pub fn fib(n: u8) -> u128 {
    fn helper(n: u8, acc1: u128, acc2: u128) -> u128 {
        if n <= 0 {
            acc2
        } else if n == 1 {
            acc1
        } else {
            helper(n - 1, acc1 + acc2, acc1)
        }
    }
    helper(n, 1, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 测试斐波那契数列
    fn fib_test() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(0), 1);
        assert_eq!(fib(2), 2);
        assert_eq!(fib(3), 3);
        assert_eq!(fib(4), 5);
    }
}

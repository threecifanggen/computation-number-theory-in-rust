/// 欧几里得方法

/**
取模运算

用整除的方式实现
*/
fn mod_(a: u128, b: u128) -> u128 {
    a - b * quotient(a, b)
}

/**
整除运算

```
quotient(2, 1) == 2
```
*/
fn quotient(a: u128, b:u128) -> u128 {
    fn helper(a: u128, b: u128, acc: u128) -> u128 {
        if a < b {
            acc
        } else {
            helper(a - b, b, acc + 1)
        }
    }
    helper(a, b, 0)
}

pub fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, mod_(a, b))
    }
}

mod tests {
    use super::*;

    #[test]
    // 测试斐波那契数列
    fn quotient_test() {
        assert_eq!(quotient(2, 3), 0);
        assert_eq!(quotient(5, 2), 2);
        assert_eq!(quotient(4, 2), 2);
    }

    #[test]
    fn mod_test() {
        assert_eq!(mod_(3, 2), 1);
        assert_eq!(mod_(5, 2), 1);
        assert_eq!(mod_(4, 2), 0);
    }

    #[test]
    fn gcd_test(){
        assert_eq!(gcd(5, 2), 1);
        assert_eq!(gcd(12, 8), 4);
    }

}
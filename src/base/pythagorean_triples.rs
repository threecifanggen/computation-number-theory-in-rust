// 毕达哥拉斯三元组

pub fn pythagorean_triples(m: i16, n: i16) -> (i128, i128, i128) {
    (
        i128::from(m.pow(2) - n.pow(2)),
        i128::from(2 * m * n),
        i128::from(m.pow(2) + n.pow(2))
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pythagorean_triples_test () {
        let (a, b, c) = pythagorean_triples(1, 2);
        assert_eq!(a.pow(2) + b.pow(2), c.pow(2))
    }
}


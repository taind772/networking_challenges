pub fn plus(v: &[u32]) -> u32 {
    v.iter().sum()
}

pub fn poly(v: &[u32]) -> u32 {
    use std::convert::TryFrom;
    let m = u64::from(v[1]);
    let x = u64::from(v[2]);
    let x = x % m;
    let mut acc = 1;
    let result = v[3..]
        .iter()
        .map(|&i| {
            let res = u64::from(i) * acc % m;
            acc = acc * x % m;
            res
        })
        .sum::<u64>();
    u32::try_from(result % m).unwrap()
}

#[cfg(test)]
mod test {
    use crate::solve::{plus, poly};

    #[test]
    fn plus_test() {
        assert_eq!(plus(&[2, 4]), 6);
        assert_eq!(plus(&[1000, 255]), 1255);
    }

    #[test]
    fn poly_test() {
        assert_eq!(poly(&[3, 10000, 10, 1, 1, 1, 1]), 1111);
    }
}
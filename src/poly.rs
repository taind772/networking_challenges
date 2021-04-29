use networking_challenges::{constants::ADDR_POLY, run};
use std::convert::TryFrom;

fn main() {
    run(ADDR_POLY, |v| {
        let m = u64::from(v[1]);
        let x = u64::from(v[2]);
        let x = x % m;
        let mut acc = 1;
        let result = v[3..].iter().map(|&i| {
            let res = u64::from(i) * acc % m;
            acc = acc * x % m;
            res
        }).sum::<u64>();
        u32::try_from(result % m).unwrap()
    })
}

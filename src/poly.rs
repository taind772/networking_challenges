use networking_challenges::{constants::ADDR_POLY, run};

fn main() {
    run(ADDR_POLY, |v| {
        let m = v[1] as u64;
        let x = v[2] as u64;
        let x = x % m;
        let mut acc = 1;
        let result = v[3..].iter().map(|&i| {
            let res = u64::from(i) * acc % m;
            acc = acc * x % m;
            res
        }).sum::<u64>();
        (result % m) as u32
    })
}

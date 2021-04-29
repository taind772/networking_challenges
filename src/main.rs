use std::io::stdin;

use networking_challenges::{constants::{ADDR_PLUS, ADDR_POLY}, run};

fn plus(id: &str){
    run(ADDR_PLUS,id, |v| v[0] + v[1]);
}

fn poly(id: &str) {
    run(ADDR_POLY, id, |v| {
        use std::convert::TryFrom;
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



fn main() {
    let get_input = |req| {
        let mut input = String::new();
        if let Some(req) = req {
            println!("{}", req);
        }
        stdin().read_line(&mut input).expect("Couldn't read from stdin");
        input.trim().to_owned()
    };
    let id = get_input(Some("Your student id is: "));
    dbg!(&id);
    println!("Networking's challenges: \n1.plus \n2.poly");
    loop {
        match get_input(Some("Your choose (q to quit): [1, 2, q]")).as_str() {
            "1" => plus(&id),
            "2" => poly(&id),
            "q" => break,
            _ => println!("Invalid choose.")
        }
    }
}
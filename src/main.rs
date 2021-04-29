use std::io::stdin;

use networking_challenges::{
    constants::{ADDR_PLUS, ADDR_POLY},
    run,
    solve::{plus, poly},
};

fn main() {
    let get_input = |req| {
        let mut input = String::new();
        if let Some(req) = req {
            println!("{}", req);
        }
        stdin()
            .read_line(&mut input)
            .expect("Couldn't read from stdin");
        input.trim().to_owned()
    };
    let id = get_input(Some("Your student id is: "));
    dbg!(&id);
    println!("Networking's challenges: \n1.plus \n2.poly");
    loop {
        match get_input(Some("Your choose (q to quit): [1, 2, q]")).as_str() {
            "1" => run(ADDR_PLUS, &id, plus),
            "2" => run(ADDR_POLY, &id, poly),
            "q" => break,
            _ => println!("Invalid choose."),
        }
    }
}

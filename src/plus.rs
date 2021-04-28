use networking_challenges::{constants::ADDR_PLUS, run};

fn main() {
    run(ADDR_PLUS, |v| v[0] + v[1])
}

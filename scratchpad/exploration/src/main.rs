mod uniform_seq;

fn main() {
    let sequence = uniform_seq::create_random_seq(10, 0, 100);
    println!("Sequence: {:?}", sequence);
}

fn main() {
    let input = "1 2";
    println!("{:?}", input.split_whitespace().map(|part| part.parse::<usize>()));
    println!("{:?}", "a b".split_whitespace());
}

fn main() {
    let _cli = heavy::parse_cli();
    let schedule = heavy::read_config();
    println!("{:?}", schedule);
}

pub fn parse_line(input: &str) -> (usize, usize, usize) {
    let mut spliter = input.split(",");
    let x = spliter.next().unwrap().parse().unwrap();
    let y = spliter.next().unwrap().parse().unwrap();
    let z = spliter.next().unwrap().parse().unwrap();
    (x, y, z)
}

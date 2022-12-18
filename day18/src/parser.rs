use crate::structs::Point;

pub fn parse_line(input: &str) -> (usize, usize, usize) {
    let mut spliter = input.split(",");
    let x = spliter.next().unwrap().parse().unwrap();
    let y = spliter.next().unwrap().parse().unwrap();
    let z = spliter.next().unwrap().parse().unwrap();
    (x, y, z)
}

pub fn parse(content: &str) -> Vec<Point> {
    let mut points = Vec::new();
    for line in content.lines() {
        let (x, y, z) = parse_line(line);
        let mut new_point = Point::new(x, y, z);
        for point in points.iter_mut() {
            new_point.attch(point);
        }
        points.push(new_point);
    }
    points
}

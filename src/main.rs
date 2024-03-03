use std::io;

#[inline]
fn square(n: i64) -> i64 {
    n * n
}

fn solution(x1: i64, y1: i64, r1: i64, x2: i64, y2: i64, r2: i64) -> i64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    let distance_squared = square(dx) + square(dy);
    let overlap = r1 * r1 > distance_squared || r2 * r2 > distance_squared;
    return if overlap {
        if x1 == x2 && y1 == y2 {
            return if r1 == r2 { -1 } else { 0 };
        }
        let r_min_squared = square(r1.min(r2));
        let r_max_squared = square(r1.max(r2));
        if 4 * distance_squared * r_max_squared
            == square(r_min_squared - r_max_squared - distance_squared)
        // if distance + r_min == r_max
        {
            1
        } else if distance_squared < square(r1 - r2)
        // else if distance + r_min < r_max
        {
            0
        } else {
            2
        }
    } else {
        let r = r1 + r2;
        let r_squared = square(r);
        if r_squared == distance_squared {
            1
        } else if r_squared < distance_squared {
            0
        } else {
            2
        }
    };
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();

    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let vec = line
            .trim()
            .split(' ')
            .flat_map(|x| x.parse::<i64>())
            .collect::<Vec<_>>();
        let result = solution(vec[0], vec[1], vec[2], vec[3], vec[4], vec[5]);
        println!("{}", result)
    }
}

#[cfg(test)]
mod day1;

mod day2;

#[cfg(test)]
mod tests {
    use crate::day1::*;

    #[test]
    fn day1_part1() {
        let filename = "data/day1.txt";
        let data = day1_parse(filename).unwrap();

        let answer = day1_part1_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 1553);
    }

    #[test]
    fn day1_part2() {
        let filename = "data/day1.txt";
        let data = day1_parse(filename).unwrap();

        let answer = day1_part2_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 1597);
    }
}

fn day2_part1() {
    let filename = "data/day2.txt";
    let data = day2::day2_parse(filename).unwrap();

    //println!("{:?}", data);

    let answer = day2::day2_part1_solve(data);
    println!("{}", answer);
    assert_eq!(answer, 2272262);
}

fn main() {
    day2_part1();
    println!("hello world!");
}

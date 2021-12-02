#[cfg(test)]
mod day1;

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

fn main() {
    println!("hello world!");
}

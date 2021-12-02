#[cfg(test)]
mod day1;

mod day2;

#[cfg(test)]
mod tests {
    use crate::day1;
    use crate::day2;

    #[test]
    fn day1() {
        let filename = "data/day1.txt";
        let data = day1::day1_parse(filename).unwrap();

        let answer = day1::day1_part1_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 1553);

        let answer = day1::day1_part2_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 1597);
    }

    #[test]
    fn day2() {
        let filename = "data/day2.txt";
        let data = day2::day2_parse(filename).unwrap();

        //println!("{:?}", data);

        let answer = day2::day2_part1_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 2272262);

        let answer = day2::day2_part2_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 2134882034);
    }
}

fn main() {
    println!("hello world!");
}

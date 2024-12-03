fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let lines = input.trim().lines();

    let reports = lines.fold(Vec::new(), |mut acc, line| {
        //
        acc.push(
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u32>().expect("unable to parse into a u32"))
                .collect::<Vec<u32>>(),
        );
        acc
    });

    reports
}

fn report_is_safe(report: &[u32]) -> bool {
    let mut ascending = None;

    for rs in report.windows(2) {
        if !(1..=3).contains(&rs[0].abs_diff(rs[1])) {
            return false;
        }

        let diff = rs[0].checked_sub(rs[1]);
        if ascending.is_none() {
            ascending = Some(diff.is_none());
        } else if (diff.is_some() && ascending.unwrap()) || (diff.is_none() && !ascending.unwrap())
        {
            return false;
        }
    }

    true
}

fn part1(input: &str) {
    let reports = parse_input(input);

    let num_safe_reports = reports
        .into_iter()
        .map(|r| if report_is_safe(&r) { 1 } else { 0 })
        .sum::<u32>();

    println!("part 1: {}", num_safe_reports);
}

fn part2(input: &str) {
    let reports = parse_input(input);

    let num_safe_reports = reports
        .into_iter()
        .map(|r| {
            if report_is_safe(&r) {
                1
            } else {
                // FIXME: brute force??? clone??? TRASH!!!. Do Better!!!
                let mut s = 0;
                for i in 0..r.len() {
                    let mut nr = r.clone();
                    nr.remove(i);
                    if report_is_safe(&nr) {
                        s = 1;
                        break;
                    }
                }

                s
            }
        })
        .sum::<u32>();

    println!("part 2: {}", num_safe_reports);
}

fn main() {
    // let input = include_str!("../../inputs/day2.sample");
    let input = include_str!("../../inputs/day2.input");
    part1(input);
    part2(input);
}

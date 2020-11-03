use std::fs;

fn parse_line(l: &str) -> Vec<usize> {
    l.split(" ")
        .filter(|v| v.trim().len() > 0)
        .map(|p| p.trim().parse::<usize>().unwrap())
        .collect()
}

fn get_triangles(input: &str) -> Vec<Vec<usize>> {
    input.trim().lines().map(|l| parse_line(l)).collect()
}

fn get_triangles_v2(input: &str) -> Vec<Vec<usize>> {
    let initial_set = get_triangles(input);
    let mut transformed: Vec<Vec<usize>> = Vec::new();

    for (i, line1) in initial_set.iter().enumerate() {
        if i % 3 != 0 {
            continue;
        }

        let line2 = initial_set.get(i + 1).unwrap();
        let line3 = initial_set.get(i + 2).unwrap();

        for i in 0..3 {
            transformed.push(vec![line1[i], line2[i], line3[i]]);
        }
    }

    transformed
}

fn count_valid(data: Vec<Vec<usize>>) -> usize {
    data.into_iter()
        .filter(|triangle| {
            triangle.iter().enumerate().all(|(i, side_1)| {
                let side_2 = triangle[(i + 1) % 3];
                let side_3 = triangle[(i + 2) % 3];

                *side_1 + side_2 > side_3
            })
        })
        .count()
}

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").unwrap();

    println!("{}", count_valid(get_triangles(&input)));
    println!("{}", count_valid(get_triangles_v2(&input)));
}

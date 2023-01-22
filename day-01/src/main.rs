use std::fs;
fn get_file(file_name: &str) -> Vec<usize> {
    let data_file = fs::read_to_string(file_name).expect("Unable to read the file");

    let data = data_file.split("\n\n").collect::<Vec<_>>();
    let data: Vec<String> = data.iter().map(|s| s.to_string()).collect();
    data.iter()
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|num_str| num_str.parse::<usize>().expect("This is not a number"))
                .sum::<usize>()
        })
        .collect()
}
fn part_one(mut counts: Vec<usize>) {
    counts.sort_by(|a, b| b.cmp(a));
    println!(
        "Part one result is: {:?}",
        counts
            .get(0)
            .expect("Cannot get first(largest) element in the vector")
    )
}

fn part_two(mut counts: Vec<usize>) {
    counts.sort_by(|a, b| b.cmp(a));
    let sum = vec![
        counts
            .get(0)
            .expect("Cannot get first(largest) element in the vector")
            .to_owned(),
        counts
            .get(1)
            .expect("Cannot get second(second largest) element in the vector")
            .to_owned(),
        counts
            .get(2)
            .expect("Cannot get third( hird largest) element in the vector")
            .to_owned(),
    ]
    .iter()
    .sum::<usize>();
    println!("Part 2 result is: {}", sum)
}

fn main() {
    let data = get_file("./data.txt");
    part_one(data.clone());
    part_two(data.clone());
}

use day_03::group::Group;

fn get_file_string(file_path: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    contents
}
fn main() {
    let file_string = get_file_string("test1.txt");
    let mut sum: usize = 0;
    // for line in file_string.lines() {
    //     let ruststack = line.parse::<Ruststack>().unwrap();
    //     sum += ruststack.return_priority() as usize;
    //     println!(
    //         "{}, {}",
    //         ruststack.find_common().unwrap(),
    //         ruststack.return_priority() as usize
    //     );
    // }
    let file_vec: Vec<_> = file_string.lines().collect();
    for i in file_vec.chunks(3) {
        let group = Group::new(i[0].to_string(), i[1].to_string(), i[2].to_string());
        sum += group.return_priority() as usize;
    }
    println!("{}", sum)
}

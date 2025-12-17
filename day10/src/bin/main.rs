use day10::parse::parse_file;

const FILE_INPUT: &str = "input-test.txt";

fn main() {
    let machines = parse_file(&FILE_INPUT);

    // FIRST PART
    let mut res = 0;
    for machine in machines.iter() {
        res += machine.min_button_light();
    }
    println!("Total num of buttons: {}", res);

    // SECOND PART
    // let mut res = 0;
    // for machine in machines.iter() {
    //     res += machine.min_button_joltage_bis();
    // }
    // println!("Total num of buttons: {}", res);
}

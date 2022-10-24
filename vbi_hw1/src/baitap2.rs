use std::io;

pub fn bai_tap2() {
    let input = "adbcdaDd";
    let mut target = String::new();

    println!("Enter a character: ");

    io::stdin()
        .read_line(&mut target)
        .expect("can not read line");
    
    let target: char = target.trim().parse().expect("you did not enter a char!");
    
    let mut count = 0;
    let mut output = String::new();
    println!("{}", target);
    for ch in input.chars().enumerate() {
        if ch.1 == target {
            count = count+1;
        }
        else {
            output.push(ch.1);
        }
    }

    assert_eq!(count ,2);
    println!("{}, {}", count, output);
}
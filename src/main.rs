fn main() {
    println!("Hello, world!");

    let input = include_str!("../input.txt");
    let vals = input.split("\n");

    let mut prev: i32 = -1;
    let mut up: Vec<i32> = vec![];
    let mut dn: Vec<i32> = vec![];

    for val_str in vals {
        if let Ok(val) = val_str.trim().parse::<i32>() {
            if prev == -1 {
                // First value.  Skip this loop.
            } else if val > prev {
                print!("+");
                up.push(val);
            } else {
                print!("-");
                dn.push(val);
            }

            prev = val;
        };
    }
    println!();
    println!("up.len(): {}", up.len());
    println!("dn.len(): {}", dn.len());
}

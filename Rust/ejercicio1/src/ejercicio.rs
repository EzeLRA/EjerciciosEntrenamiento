use std::io;

fn main() {
    let mut read_in = String::new();
    io::stdin().read_line(&mut read_in).expect("Read Error");

    let w : u8 = read_in.trim().parse().expect("Must be insert a number");

    if w > 2 {
        let piece = w / 2;
        if ((w % 2) == 0)&&(piece % 2 == 0) {
            println!("YES");
        }else{
            println!("NO");
        }
    }else{
        println!("NO");
    }

}

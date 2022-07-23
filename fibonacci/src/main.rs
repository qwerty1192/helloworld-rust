use std::io;

fn main() {
    println!("Hello, world!");

    loop{
        let mut n = String::new();

        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        if n == "exit\n"{
            break;
        }
        
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let x: u128= fibonacci(&n);

        if (10..=19).contains(&n){
            println!("The {}th number is: {}", n, x);
        }else if n%10 == 1{
            println!("The {}st number is: {}", n, x);
        }else if n%10 == 2{
            println!("The {}nd number is: {}", n, x);
        }else if n%10 == 3{
            println!("The {}rd number is: {}", n, x);
        }else{
            println!("The {}th number is: {}", n, x);
        }
    }
}

fn fibonacci(&n: &u32) -> u128 {
    let mut a: u128 = 1;
    let mut b: u128 = 1;
    
    
    if n >= 3 {
        let mut c: u128;
        for _number in 3..=n{
            c = a + b;
            a = b;
            b = c;
        }
    }

    b
}

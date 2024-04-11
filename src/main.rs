// use std::vec;

fn main() {
    // let mut a = vec![];
    // let mut b = a;

    // b.push(1);
    // a.push(1);

    let mut x = 5; 

    {
        let y = &mut x;

        *y = 6;

        println!("{:?}",x);
    }

    x=9;

    println!("{:?}",x);

    // let mut a:Vec<i32> = vec![];

    // let b = &mut a;

    // let c = &a;


}

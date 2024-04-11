// use std::vec;

enum RSEnum{
    Foo2(Option<i32>),
    Foo(fn()->i32), 
    Bar(String), 
    Baz(Vec<String>),
}

fn bar()->i32{
    return 5;
}

fn lec1() {
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

fn lec2(){
    let foo = RSEnum::Foo(bar);

    match foo {
        RSEnum::Foo2(Some(value)) => print!("{}",value),
        RSEnum::Foo2(None) => {},
        RSEnum::Foo(value) => print!("{}",value),
        _ => {}
    }
}

fn lec2_1(){
    let foo = Some(5);

    if let Some(value) = foo {
        print!("{}",value);
    }

    match foo{
        Some(value) => print!("{}",value),
        None => {}
    };

    foo.map(|x| {
        print!("{}",x);
    });

    foo.filter(|x| x < &10 );
}

fn main() {
    // lec1();
    // lec2();
    lec2_1()
}
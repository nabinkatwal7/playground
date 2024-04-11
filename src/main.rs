// use std::vec;

// enum RSEnum{
//     Foo2(Option<i32>),
//     Foo(fn()->i32), 
//     // Bar(String), 
//     // Baz(Vec<String>),
// }

// fn bar()->i32{
//     return 5;
// }

// fn lec1() {
//     // let mut a = vec![];
//     // let mut b = a;

//     // b.push(1);
//     // a.push(1);

//     let mut x = 5; 

//     {
//         let y = &mut x;

//         *y = 6;

//         println!("{:?}",x);
//     }

//     x=9;

//     println!("{:?}",x);

//     // let mut a:Vec<i32> = vec![];

//     // let b = &mut a;

//     // let c = &a;

// }

// fn lec2(){
//     let foo = RSEnum::Foo(bar);

//     match foo {
//         RSEnum::Foo2(Some(value)) => print!("{}",value),
//         RSEnum::Foo2(None) => {},
//         _ => {}
//     }
// }

// fn lec2_1(){
//     let foo = Some(5);

//     if let Some(value) = foo {
//         print!("{}",value);
//     }

//     match foo{
//         Some(value) => print!("{}",value),
//         None => {}
//     };

//     foo.map(|x| {
//         print!("{}",x);
//     });

//     foo.filter(|x| x < &10 );
// }

fn lec3(){
    enum Option2<T>{
        None,
        Some(T),
    }

    impl<T> Option2<T>{
        fn is_some(&self)->bool{
            return match *self {
                Option2::None => false, 
                Option2::Some(_) => true
            };
        }

        fn unwrap(self)->T{
            match self {
                Option2::None => panic!("called `Option::unwrap()` on a `None` value"),
                Option2::Some(value) => value
            }
        }
    }

    let foo = Option2::Some(5);

    if foo.is_some() {
        print!("{}",foo.unwrap());
    }
}

fn main() {
    // lec1();
    // lec2();
    // lec2_1();
    // lec3();
    lec3();
}
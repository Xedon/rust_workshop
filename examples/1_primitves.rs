fn main() {
    let str = "str";
    let string = String::from("string");
    let int = 999999999;
    let double = 1.;
    let bool = false;

    // {} display formatting
    println!("{} + {} = {}", int, double, double + (int as f64)); // as xx is a typecast
    println!("{} && {} = {}", bool, false, bool && false);

    // {} display formatting seit version 2021
    println!("{int} + {double} = 1000000000");
    println!("{bool} && false = false");

    let tuple = (10, "Zehn", 10.);

    // :#? = debug formatting
    println!("{:#?}", tuple);
    println!("{}", tuple.1);

    let array = [1, 2, 3];

    let array2 = [1; 5];

    println!("{:#?} \nlen: {}", array, array.len());

    println!("{:#?}", array2);

    let array3 = [&array[1..2], &array2[0..2]];

    println!("{:#?}", array3);

    let x = 1;

    let x = "eins";

    println!("{}", x);

    let x: u64;
    // println!("{}", x); // doesn't compile because it is not initialized
    x = 123;

    println!("{}", x);

    let mut x = "a";

    x = "b";

    println!("{}", x);

    {
        let x = x;
        // x = "abc";  // doesn't compile because x is now immutable in this scope
    }

    // now the immutable var is out of scope
    x = "c";

    println!("{}", x);
}

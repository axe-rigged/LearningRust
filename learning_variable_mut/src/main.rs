fn main() {
    let mut x = 5;
    println!("The value is {x}");
    x = 6;
    println!("The value is {x}");

    let y = 5;
    let y = y + 1;

    {
        let y = y * 3;
        println!("shadowed y is {y}");
    }

    println!("reused y is {y}");
    //let can be used to change types. let mut only lets you change value of the change type what
    //it was asigned.
    
    //want to test if new variable share value from mut array or without. But looks like it assign
    //own value for c when we create it. With &mut would share.
    let mut l = [1,2,3,4,5];
    
    let c = l[3];
    
    l[3] = 6;
    
    let ln = l[3];

    println!("l is fourth index value is {ln}. C value is {c}");

    my_own_function(5);

    let g = give_number();
    println!("G is {g}");

    let h = returning_loop();
    println!("h should be 10 == {h}");

    let p = multi_if_return(5);
    let q = multi_if_return(20);
    let r = multi_if_return(10);

    println!("p should be 10 == {p}");
    println!("q should be 19 == {q}");
    println!("r should be {r}");
}

fn my_own_function(x: i32) {
    println!("Print {x}");
}

fn give_number() -> u8 {
    100
}

fn returning_loop() -> i32 {
    let mut x: i32 = 0;
    loop {
        x = x + 1;
        if x == 10 {
            break x;
        }
    }
}

fn multi_if_return(x: i32) -> i32 {
    if x < 10 {
        return x * 2;
    }
    if x > 10 {
        return x - 1;
    }
    else {
        x
    }
}

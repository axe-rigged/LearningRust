//Brings code to our scope.
use creatures::enemy::Zombie;

//everything is private inside module. IF something is bringed with
//mod everything that you want to use need to be pub.
//You can use private values and use public function to get values.
mod creatures;

fn main() {
    let enemy = Zombie {
        hp: 8,
        name: String::from("Underworld"),
    };

    println!("{} {}", enemy.name, enemy.hp);

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let y = &v[2];
    println!("Y should be 3, {y}");

    //When we are getting value from list/vector that is out of bound and we dont know that we should use Vec.Get().
    //This way we can check is there value and handel error.
    let seco: Option<&i32> = v.get(1);
    match seco {
        Some(seco) => println!("There was value and its {seco}"),
        None => println!("No value was found"),
    }

    let mut dog = String::from("Dog");
    dog.push_str(" is animal");
    println!("My string: {}", dog);
    //format!("{string 1} {string2}") lets ass format string inside "" and parse value to variable
}

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
}

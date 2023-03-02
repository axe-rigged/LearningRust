struct Enemy {
    alive: bool,
    name: String,
    hp: u8,
}

//We can create tuple structure for no need to name fields.
//struct Position(i32, i32, i32);

fn main() {
    let enemy1 = Enemy {
        alive: true,
        name: String::from("Skeleton"),
        hp: 8,
    };

    let mut enemy2 = Enemy {
        alive: true,
        name: String::from("Skeleton"),
        hp: 8,
    };

    enemy2.name = String::from("Bat");
    if enemy1.alive == true || enemy2.alive == true {
        println!("Enemys living: {0}, {1}.\nEnemys hp: {2}, {3}", enemy1.name, enemy2.name, enemy1.hp, enemy2.hp);
    }
}

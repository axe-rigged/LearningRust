struct Enemy {
    alive: bool,
    name: String,
    hp: u8,
}

//Not need to reference number because its fast copy.
//impl Enemy {
//  fn attack(&self, mod: u8) -> u8 {
//      self.attack * mod
//  }
//}

//We can create tuple structure for no need to name fields.
//#[derive(Debug)]
//struct Position(i32, i32, i32);
//dbg!()

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
    if enemy_living(&enemy1) == true ||  enemy_living(&enemy2) == true {
        println!("Enemys living: {0}, {1}.\nEnemys hp: {2}, {3}", enemy1.name, enemy2.name, enemy1.hp, enemy2.hp);
    }

}

fn enemy_living(enemies: &Enemy) -> bool {
    enemies.alive
}

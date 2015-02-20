extern crate battlesystem;

fn main() {
  let mut battle_system = battlesystem::BTL::BattleSystem { combatants: vec![] };
  battle_system.generate_combatants(3);

  loop {
    println!("Enter command");
    let raw = std::old_io::stdin().read_line().ok().expect("Failed to read line");
    let input = raw.trim();

    if input == "quit" {
      break;
    }
    else {
      battle_system.exec_command(input.to_string());
      let combatants = &battle_system.combatants;
      if combatants[0].health <= 0 {
        println!("You have died.");
        break;
      }
      else if combatants.len() <= 1 {
        println!("You have won.");
        break;
      }
    }
  }
}
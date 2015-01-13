extern crate battlesystem;

fn main() {
  let mut battle_system = battlesystem::BTL::BattleSystem { combatants: vec![] };
  battle_system.generate_combatants(2);

  loop {
    println!("Enter command");
    let raw = std::io::stdin().read_line().ok().expect("Failed to read line");
    let input = raw.trim();

    if input == "quit" {
      break;
    }
    else {
      battle_system.exec_command(input.to_string());
    }
  }
}
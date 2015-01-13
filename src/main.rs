extern crate battlesystem;

fn main() {
  let battle_system = battlesystem::BattleSystem { targets: vec![1, 2] };
  let player_one_id = 1;

  println!("Enter command");

  loop {
    let raw = std::io::stdin().read_line().ok().expect("Failed to read line");
    let input = raw.trim();

    if input == "quit" {
      break;
    }
    else {
      battle_system.exec_command(input);
    }
  }
}
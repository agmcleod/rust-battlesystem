use BTL::BattleSystem;

pub enum Command {
  Attack,
  List,
  Help,
}

impl Command {
  pub fn invoke(&self, system: &mut BattleSystem, values: Box<Vec<&str>>) {
    match *self {
      Command::List => { system.list_combatants() },
      Command::Attack => {
        system.attack_combatant(values);
        system.invoke_ai();
        println!("After AI:");
        Command::List.invoke(system, Box::new(vec![]));
      },
      Command::Help => {
        println!("\nhelp - this command. Lists available commands");
        println!("list - lists the targets");
        println!("attack target_number - attacks the target specified by the number.\n");
      },
    }
  }
}
pub mod BTL {
  use std::collections::hash_map::HashMap;
  use std::rand;
  use self::cmd::Command;
  use self::entities::Combatant;
  mod cmd {
    pub enum Command {
      List,
    }

    impl Command {
      pub fn invoke(&self, system: &super::BattleSystem) {
        match *self {
          Command::List => { system.list_combatants() },
        }
      }
    }
  }

  mod entities {
    use std::fmt::{Formatter, Result, String};
    pub struct Combatant {
      pub health: isize,
      pub damage: isize,
    }

    impl String for Combatant {
      fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "heath: {} damage: {}", self.health, self.damage)
      }
    }
  }

  pub struct BattleSystem {
    pub combatants: Vec<Combatant>,
  }

  impl BattleSystem {
    pub fn commands() -> HashMap<String, Command> {
      let mut map: HashMap<String, Command> = HashMap::new();
      map.insert("list".to_string(), Command::List);
      map
    }

    pub fn exec_command(&self, command: String) {
      let commands = BattleSystem::commands();
      match commands.get(&command) {
        Some(ref cmd) => { cmd.invoke(self) },
        _ => println!("No command found")
      }
    }

    pub fn generate_combatants(&mut self, amount: isize) {
      if amount < 2 {
        panic!("Invalid combatant count, must be 2 or more");
      }
      for i in range(0, amount) {
        if i == 0 {
          self.combatants.push(Combatant{ health: 10, damage: 5 });
        }
        else {
          let health = (rand::random::<isize>() % 4) + 5;
          let dmg = (rand::random::<isize>() % 2) + 3;
          self.combatants.push(Combatant{ health: health, damage: dmg });
        }
      }
    }

    pub fn list_combatants(&self) {
      let mut label : &str;
      let mut i = 0;
      for c in self.combatants.iter() {
        if i == 0 {
          label = "Player";
        }
        else {
          label = "Combatant";
        }
        println!("({}) {}: {}", i + 1, label, c);
        i += 1;
      }
    }
  }
}

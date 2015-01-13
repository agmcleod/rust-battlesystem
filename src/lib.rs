pub mod BTL {
  use std::collections::hash_map::HashMap;
  use std::rand;
  use self::cmd::Command;
  use self::entities::Combatant;
  mod cmd {
    pub enum Command {
      Attack,
      List,
    }

    impl Command {
      pub fn invoke(&self, system: &mut super::BattleSystem, values: Box<Vec<&str>>) {
        match *self {
          Command::List => { system.list_combatants() },
          Command::Attack => { system.attack_combatant(values[1]); Command::List.invoke(system, Box::new(vec![])); },
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
      map.insert("attack".to_string(), Command::Attack);
      map
    }

    pub fn attack_combatant(&mut self, combatant: &str) {
      let num: Option<usize> = combatant.parse::<usize>();
      let i = num.unwrap();
      if num == None || (i - 1) < 0 || (i - 1) >= self.combatants.len() {
        println!("Invalid target");
      }
      else {
        let (player_set, enemies) = self.combatants.split_at_mut(1);
        if (i == 1) {
          let player = &mut player_set[0];
          player.health -= player.damage;
        }
        else {
          let player = &player_set[0];
          let enemy = &mut enemies[i - 2];
          enemy.health -= player.damage;
        }
      }
    }

    pub fn exec_command(&mut self, command: String) {
      let split = command.split_str(" ");
      let values : Vec<&str> = split.collect();
      let commands = BattleSystem::commands();
      match commands.get(&values[0].to_string()) {
        Some(ref cmd) => { cmd.invoke(self, Box::new(values)) },
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

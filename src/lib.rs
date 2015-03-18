pub mod BTL {
  use std::collections::hash_map::HashMap;
  use std::rand;
  use self::cmd::Command;
  use self::entities::Combatant;
  use std::num::ParseIntError;
  use std::fmt;
  mod cmd {
    pub enum Command {
      Attack,
      List,
      Help,
    }

    impl Command {
      pub fn invoke(&self, system: &mut super::BattleSystem, values: Box<Vec<&str>>) {
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
  }

  mod entities {
    pub struct Combatant {
      pub health: isize,
      pub damage: isize,
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
      map.insert("help".to_string(), Command::Help);
      map
    }

    pub fn attack_combatant(&mut self, arguments: Box<Vec<&str>>) {
      if (arguments.len() < 2) {
        println!("Must specify a target");
        return;
      }
      let combatant = arguments[1];
      let num: Result<usize, ParseIntError> = combatant.parse::<usize>();
      let i = num.ok().unwrap();
      let mut remove = false;
      if (i - 1) < 0 || (i - 1) >= self.combatants.len() {
        println!("Invalid target. Pass corresponding number:");
      }
      else {
        let damage: isize;
        {
          let player = &self.combatants[0];
          damage = player.damage;
        };

        let target = &mut self.combatants[i - 1];
        target.health -= damage;
        if target.health <= 0 && i > 0 {
          remove = true;
        }
      }

      if remove {
        self.combatants.remove(i - 1);
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

    pub fn invoke_ai(&mut self) {
      let (player_set, enemies) = self.combatants.split_at_mut(1);
      let player = &mut player_set[0];
      for enemy in enemies.iter() {
        if enemy.health > 0 {
          player.health -= enemy.damage;
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
        println!("({}) {}: {}", i + 1, label, format_args!("health: {} damage: {}", c.health, c.damage));
        i += 1;
      }
    }
  }
}

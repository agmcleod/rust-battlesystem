pub mod BTL {
  use std::collections::hash_map::HashMap;
  use self::cmd::Command;
  mod cmd {
    pub enum Command {
      Targets,
    }

    impl Command {
      pub fn invoke(&self, system: &super::BattleSystem) {
        match *self {
          Command::Targets => { system.print_targets() }
        }
      }
    }
  }
  pub struct BattleSystem {
    pub targets: Vec<isize>
  }

  impl BattleSystem {
    pub fn commands() -> HashMap<String, Command> {
      let mut map: HashMap<String, Command> = HashMap::new();
      map.insert("targets".to_string(), Command::Targets);
      map
    }
    pub fn exec_command(&self, command: String) {
      let commands = BattleSystem::commands();
      match commands.get(&command) {
        Some(ref cmd) => { cmd.invoke(self) },
        _ => println!("No command found")
      }
    }
    pub fn print_targets(&self) {
      println!("Targets: {:?}", self.targets);
    }
  }
}

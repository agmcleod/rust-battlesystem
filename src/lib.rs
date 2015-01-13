use std::collections::hash_map::HashMap;

pub struct BattleSystem {
  pub command_map: HashMap<String, usize>,
  pub targets: Vec<isize>
}

impl BattleSystem {
  pub fn exec_command(&self, command: &str) {
    self.print_targets();
  }
  pub fn print_targets(&self) {
    println!("Targets: {:?}", self.targets);
  }
}
use crate::{Identifiable, Identifier, ModuleIdentifier};

pub struct ModuleGraphModule {
  /// Module identifier of this module.
  module_identifier: ModuleIdentifier,
}

impl ModuleGraphModule {
  pub fn new(module_identifier: ModuleIdentifier) -> Self {
    Self { module_identifier }
  }
}

impl Identifiable for ModuleGraphModule {
  fn identifier(&self) -> Identifier {
    self.module_identifier
  }
}

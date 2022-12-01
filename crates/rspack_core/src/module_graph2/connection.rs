use ustr::Ustr;

use crate::{Identifiable, Identifier};

pub struct ModuleGraphConnection {
  parent_module_identifier: Ustr,
  module_identifier: Ustr,
  dependency_identifier: Ustr,
  connection_identifier: Ustr,
}

impl ModuleGraphConnection {
  fn new(
    parent_module_identifier: Ustr,
    dependency_identifier: Ustr,
    module_identifier: Ustr,
  ) -> Self {
    Self {
      parent_module_identifier,
      module_identifier,
      connection_identifier: (parent_module_identifier.to_owned() + &module_identifier).into(),
      dependency_identifier,
    }
  }
}

impl ModuleGraphConnection {
  pub fn parent_module_identifier(&self) -> &Ustr {
    &self.parent_module_identifier
  }

  pub fn module_identifier(&self) -> &Ustr {
    &self.module_identifier
  }

  pub fn dependency_identifier(&self) -> &Ustr {
    &self.dependency_identifier
  }
}

impl Identifiable for ModuleGraphConnection {
  fn identifier(&self) -> Identifier {
    self.connection_identifier
  }
}

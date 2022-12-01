use std::any::Any;

use ustr::Ustr;

use crate::{AsAny, DynHash, Identifiable};

pub trait Dependency: AsAny + DynHash + Identifiable {
  fn parent_module_identifier(&self) -> &Ustr;
}

pub trait ModuleDependency: Dependency {
  fn request(&self) -> &str;
  fn user_request(&self) -> &str;
}

impl dyn Dependency + '_ {
  pub fn downcast_ref<D: Any>(&self) -> Option<&D> {
    self.as_any().downcast_ref::<D>()
  }

  pub fn downcast_mut<D: Any>(&mut self) -> Option<&mut D> {
    self.as_any_mut().downcast_mut::<D>()
  }
}

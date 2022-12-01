use std::{collections::HashMap, fmt::Display};

use petgraph::{
  graph::{DiGraph, Edges, NodeIndex},
  Directed, Direction,
};
use ustr::Ustr;

use rspack_error::{internal_error, Error, InternalError, Result};

use crate::{BoxModule, Identifiable, Module, ModuleIdentifier};

use super::{ModuleDependency, ModuleGraphConnection, ModuleGraphModule};

type MgcIdentifier = Ustr;
type DepIdentifier = Ustr;
type Connections<'e> = Edges<'e, MgcIdentifier, Directed>;

pub struct ModuleGraph {
  inner: DiGraph<ModuleIdentifier, MgcIdentifier>,
  module_identifier_to_node_index: HashMap<ModuleIdentifier, NodeIndex>,
  module_map: HashMap<ModuleIdentifier, Box<dyn Module>>,

  module_identifier_to_mgm: HashMap<ModuleIdentifier, ModuleGraphModule>,
  mgc_identifier_to_mgc: HashMap<MgcIdentifier, ModuleGraphConnection>,
  dep_identifier_to_dep: HashMap<DepIdentifier, Box<dyn ModuleDependency>>,
  dep_identifier_to_mgc_identifier: HashMap<DepIdentifier, MgcIdentifier>,
  mgc_identifier_to_dep_identifier: HashMap<MgcIdentifier, DepIdentifier>,
}

impl Default for ModuleGraph {
  fn default() -> Self {
    Self {
      inner: Default::default(),
      module_identifier_to_node_index: Default::default(),
      module_map: Default::default(),

      module_identifier_to_mgm: Default::default(),
      mgc_identifier_to_mgc: Default::default(),
      dep_identifier_to_dep: Default::default(),
      mgc_identifier_to_dep_identifier: Default::default(),
      dep_identifier_to_mgc_identifier: Default::default(),
    }
  }
}

impl ModuleGraph {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_module(&mut self, module: Box<dyn Module>) -> Ustr {
    todo!()
  }

  pub fn add_module_graph_module(&mut self, mgm: ModuleGraphModule) -> ModuleIdentifier {
    let module_identifier = mgm.identifier();
    let node_index = self.inner.add_node(module_identifier);

    self.module_identifier_to_mgm.insert(module_identifier, mgm);
    self
      .module_identifier_to_node_index
      .insert(module_identifier, node_index);

    module_identifier
  }

  pub fn add_dependency(&mut self, dep: Box<dyn ModuleDependency>) -> DepIdentifier {
    let dep_identifier = dep.identifier();
    self.dep_identifier_to_dep.insert(dep_identifier, dep);
    dep_identifier
  }

  pub fn add_connection(
    &mut self,
    from: &ModuleIdentifier,
    to: &ModuleIdentifier,
    mgc: ModuleGraphConnection,
  ) -> Result<MgcIdentifier> {
    let mgc_identifier = mgc.identifier();
    let from = self
      .module_identifier_to_node_index
      .get(&from)
      .ok_or_else(|| {
        Error::InternalError(internal_error!(format!("Failed to get connection {from}")))
      })?;
    let to = self
      .module_identifier_to_node_index
      .get(&to)
      .ok_or_else(|| {
        Error::InternalError(internal_error!(format!("Failed to get connection {to}")))
      })?;

    self.inner.add_edge(*from, *to, mgc_identifier);
    self.mgc_identifier_to_mgc.insert(mgc_identifier, mgc);

    Ok(mgc_identifier)
  }

  pub fn link_connection_and_dependency(
    &mut self,
    dep_identifier: &DepIdentifier,
    mgc_identifier: &MgcIdentifier,
  ) {
    debug_assert_eq!(
      self
        .dep_identifier_to_dep
        .get(dep_identifier)
        .and_then(|dep| {
          self
            .mgc_identifier_to_mgc
            .get(mgc_identifier)
            .and_then(|mgc| Some(mgc.parent_module_identifier() == dep.parent_module_identifier()))
        }),
      Some(true)
    );
    self
      .mgc_identifier_to_dep_identifier
      .insert(*mgc_identifier, *dep_identifier);
    self
      .dep_identifier_to_mgc_identifier
      .insert(*dep_identifier, *mgc_identifier);
  }

  pub fn module_by_dependency(&self, dep_identifier: &DepIdentifier) -> Option<&ModuleGraphModule> {
    self
      .dep_identifier_to_mgc_identifier
      .get(dep_identifier)
      .and_then(|mgc_identifier| {
        self
          .mgc_identifier_to_mgc
          .get(mgc_identifier)
          .and_then(|mgc| self.module_identifier_to_mgm.get(mgc.module_identifier()))
      })
  }

  /// Return an unordered iterator of module graph modules
  pub fn module_graph_modules(&self) -> impl Iterator<Item = &ModuleGraphModule> {
    self.module_identifier_to_mgm.values()
  }

  /// Return an unordered iterator of modules
  pub fn modules(&self) -> impl Iterator<Item = &BoxModule> {
    self.module_map.values()
  }

  /// Uniquely identify a module by its identifier and return the aliased reference
  #[inline]
  pub fn module_by_identifier(&self, identifier: &ModuleIdentifier) -> Option<&BoxModule> {
    self.module_map.get(identifier)
  }

  /// Uniquely identify a module by its identifier and return the exclusive reference
  #[inline]
  pub fn module_by_identifier_mut(
    &mut self,
    identifier: &ModuleIdentifier,
  ) -> Option<&mut BoxModule> {
    self.module_map.get_mut(identifier)
  }

  /// Uniquely identify a module graph module by its module's identifier and return the aliased reference
  #[inline]
  pub fn module_graph_module_by_identifier(
    &self,
    identifier: &ModuleIdentifier,
  ) -> Option<&ModuleGraphModule> {
    self.module_identifier_to_mgm.get(identifier)
  }

  /// Uniquely identify a module graph module by its module's identifier and return the exclusive reference
  #[inline]
  pub fn module_graph_module_by_identifier_mut(
    &mut self,
    identifier: &ModuleIdentifier,
  ) -> Option<&mut ModuleGraphModule> {
    self.module_identifier_to_mgm.get_mut(identifier)
  }

  /// Uniquely identify a connection by a given dependency
  pub fn connection_by_dependency(&self, dep: &DepIdentifier) -> Option<&ModuleGraphConnection> {
    self
      .dep_identifier_to_mgc_identifier
      .get(dep)
      .and_then(|mgc_identifier| self.mgc_identifier_to_mgc.get(mgc_identifier))
  }

  pub fn incoming_connections_of_mgm(
    &self,
    module_identifier: &ModuleIdentifier,
  ) -> Connections<'_> {
    let node_index = self
      .module_identifier_to_node_index
      .get(module_identifier)
      .expect(&format!(
        "Failed to get module graph module {module_identifier}"
      ));
    self.inner.edges_directed(*node_index, Direction::Incoming)
  }

  pub fn outgoing_connections_of_mgm(
    &self,
    module_identifier: &ModuleIdentifier,
  ) -> Connections<'_> {
    let node_index = self
      .module_identifier_to_node_index
      .get(module_identifier)
      .expect(&format!(
        "Failed to get module graph module {module_identifier}"
      ));
    self.inner.edges_directed(*node_index, Direction::Outgoing)
  }
}

impl Display for ModuleGraph {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    #[cfg(feature = "mermaid")]
    {}
    todo!()
  }
}

#![feature(anonymous_lifetime_in_impl_trait)]

mod diagnostic;
mod error;
pub use diagnostic::*;
pub use error::*;
pub mod emitter;

pub type Result<T> = std::result::Result<T, Error>;

pub type TWithDiagnosticArray<T> = TWithGenericDiagnosticArray<T, Diagnostic>;
/// A helper struct for change logic from
/// return something to something with diagnostics array
#[derive(Debug)]
pub struct TWithGenericDiagnosticArray<T: std::fmt::Debug, D> {
  pub inner: T,
  pub diagnostic: Vec<D>,
}

impl<T: std::fmt::Debug, D> TWithGenericDiagnosticArray<T, D> {
  pub fn new(inner: T, diagnostic: Vec<D>) -> Self {
    Self { inner, diagnostic }
  }

  pub fn diagnostics(&self) -> &Vec<D> {
    &self.diagnostic
  }

  pub fn take_inner(self) -> T {
    self.inner
  }

  pub fn split_into_parts(mut self) -> (T, Vec<D>) {
    let diagnostic = std::mem::take(&mut self.diagnostic);
    (self.inner, diagnostic)
  }

  pub fn map<R, F>(self, f: F) -> TWithGenericDiagnosticArray<R, D>
  where
    R: std::fmt::Debug,
    F: Fn(T) -> R,
  {
    TWithGenericDiagnosticArray {
      inner: f(self.inner),
      diagnostic: self.diagnostic,
    }
  }

  pub fn map_error<R, F>(self, f: F) -> TWithGenericDiagnosticArray<T, R>
  where
    R: std::fmt::Debug,
    F: Fn(Vec<D>) -> Vec<R>,
  {
    TWithGenericDiagnosticArray {
      inner: self.inner,
      diagnostic: f(self.diagnostic),
    }
  }
}

impl<T: Clone + std::fmt::Debug> Clone for TWithDiagnosticArray<T> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone(),
      diagnostic: self.diagnostic.clone(),
    }
  }
}

// Helper trait to make `TWithDiagnosticArray` convertion more easily.
pub trait IntoTWithDiagnosticArray {
  fn with_diagnostic(self, diagnostic: Vec<Diagnostic>) -> TWithDiagnosticArray<Self>
  where
    Self: Sized + std::fmt::Debug;

  fn with_empty_diagnostic(self) -> TWithDiagnosticArray<Self>
  where
    Self: Sized + std::fmt::Debug,
  {
    TWithDiagnosticArray {
      inner: self,
      diagnostic: vec![],
    }
  }
}

pub trait IntoTWithGenericDiagnosticArray<D> {
  fn with_generic_diagnostic(self, diagnostic: Vec<D>) -> TWithGenericDiagnosticArray<Self, D>
  where
    Self: Sized + std::fmt::Debug;

  fn with_empty_diagnostic(self) -> TWithGenericDiagnosticArray<Self, D>
  where
    Self: Sized + std::fmt::Debug,
  {
    TWithGenericDiagnosticArray {
      inner: self,
      diagnostic: vec![],
    }
  }
}

impl<T: Sized + std::fmt::Debug> IntoTWithDiagnosticArray for T {
  fn with_diagnostic(self, diagnostic: Vec<Diagnostic>) -> TWithDiagnosticArray<Self>
  where
    Self: Sized + std::fmt::Debug,
  {
    TWithDiagnosticArray {
      inner: self,
      diagnostic,
    }
  }
}

impl<T: Sized + std::fmt::Debug, D> IntoTWithGenericDiagnosticArray<D> for T {
  fn with_generic_diagnostic(self, diagnostic: Vec<D>) -> TWithGenericDiagnosticArray<Self, D>
  where
    Self: Sized + std::fmt::Debug,
  {
    TWithGenericDiagnosticArray {
      inner: self,
      diagnostic,
    }
  }
}

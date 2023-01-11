use std::path::Path;
use std::sync::Arc;

use rspack_core::{ast::javascript::Ast, ModuleType};
use rspack_error::{
  Error, IntoTWithDiagnosticArray, IntoTWithGenericDiagnosticArray, Severity, TWithDiagnosticArray,
  TWithGenericDiagnosticArray,
};
use swc_core::base::config::IsModule;
use swc_core::common::comments::Comments;
use swc_core::common::{FileName, SourceFile};
use swc_core::ecma::ast::{self, EsVersion, Program};
use swc_core::ecma::parser::{
  self, parse_file_as_module, parse_file_as_program, parse_file_as_script, Syntax,
};

use crate::utils::{ecma_parse_error_to_rspack_error, syntax_by_module_type};

/// Why this helper function design like this?
/// 1. `swc_ecma_parser` could return ast with some errors which are recoverable
/// or warning (though swc defined them as errors), but the parser at here should
/// be non-error-tolerant.
///
/// 2. We can't convert to [rspack_error::Error] at this point, because there is
/// no `path` and `source`
pub fn parse_js(
  fm: Arc<SourceFile>,
  target: EsVersion,
  syntax: Syntax,
  is_module: IsModule,
  comments: Option<&dyn Comments>,
) -> Result<
  TWithGenericDiagnosticArray<Program, (parser::error::Error, rspack_error::Severity)>,
  Vec<(parser::error::Error, rspack_error::Severity)>,
> {
  let mut errors = vec![];
  let program_result = match is_module {
    IsModule::Bool(true) => {
      parse_file_as_module(&fm, syntax, target, comments, &mut errors).map(Program::Module)
    }
    IsModule::Bool(false) => {
      parse_file_as_script(&fm, syntax, target, comments, &mut errors).map(Program::Script)
    }
    IsModule::Unknown => parse_file_as_program(&fm, syntax, target, comments, &mut errors),
  };

  //
  let mut normalized_errors = errors
    .into_iter()
    .map(|error| (error, Severity::Warn))
    .collect::<Vec<_>>();
  // Using combinator will let rustc unhappy.
  match program_result {
    Ok(program) => Ok(program.with_generic_diagnostic(normalized_errors)),
    Err(err) => {
      // This is unrecovered_error
      normalized_errors.push((err, Severity::Error));
      Err(normalized_errors)
    }
  }
}

pub fn parse(
  source_code: String,
  syntax: Syntax,
  filename: &str,
  module_type: &ModuleType,
) -> Result<TWithGenericDiagnosticArray<Ast, (parser::error::Error, rspack_error::Severity)>, Error>
{
  let source_code = if syntax.dts() {
    // dts build result must be empty
    "".to_string()
  } else {
    source_code
  };

  let cm: Arc<swc_core::common::SourceMap> = Default::default();
  let fm = cm.new_source_file(FileName::Custom(filename.to_string()), source_code);

  match parse_js(
    fm,
    ast::EsVersion::Es2022,
    syntax,
    // TODO: Is this correct to think the code is module by default?
    IsModule::Bool(true),
    None,
  ) {
    Ok(parse_result) => Ok(parse_result.map(|inner| Ast::new(inner, cm))),
    Err(errs) => Err(Error::BatchErrors(
      errs
        .into_iter()
        .map(|(err, severity)| {
          ecma_parse_error_to_rspack_error(err, filename, module_type, severity)
        })
        .collect::<Vec<_>>(),
    )),
  }
}

pub fn parse_js_code(js_code: String, module_type: &ModuleType) -> Result<Program, Error> {
  let filename = "".to_string();
  let syntax = syntax_by_module_type(Path::new(&filename), module_type, false);
  let cm: Arc<swc_core::common::SourceMap> = Default::default();
  let fm = cm.new_source_file(FileName::Custom(filename), js_code);

  match parse_js(
    fm,
    ast::EsVersion::Es2022,
    syntax,
    // TODO: Is this correct to think the code is module by default?
    IsModule::Bool(true),
    None,
  ) {
    Ok(program) => Ok(program),
    Err(errs) => Err(Error::BatchErrors(
      errs
        .into_iter()
        .map(|err| ecma_parse_error_to_rspack_error(err, "", module_type))
        .collect::<Vec<_>>(),
    )),
  }
}

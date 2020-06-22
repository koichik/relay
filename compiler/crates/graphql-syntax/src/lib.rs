/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#![deny(warnings)]
#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![allow(clippy::large_enum_variant)]

mod char_constants;
mod lexer;
mod lexer_position;
mod parser;
mod source;
mod syntax_error;
mod syntax_node;
mod token_kind;
pub use source::GraphQLSource;
pub use syntax_error::{SyntaxError, SyntaxErrorKind, SyntaxErrorWithSource};
pub use syntax_node::*;

use crate::parser::Parser;
use common::SourceLocationKey;

pub fn parse(source: &str, source_location: SourceLocationKey) -> SyntaxResult<Document> {
    let parser = Parser::new(source, source_location);
    parser.parse_document()
}

pub fn parse_type(
    source: &str,
    source_location: SourceLocationKey,
) -> SyntaxResult<TypeAnnotation> {
    let parser = Parser::new(source, source_location);
    parser.parse_type()
}

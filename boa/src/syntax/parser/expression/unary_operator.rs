//! Unary operator parsing.
//!
//! More information:
//!  - [MDN documentation][mdn]
//!  - [ECMAScript specification][spec]
//!
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Expressions_and_Operators#Unary
//! [spec]: https://tc39.es/ecma262/#sec-unary-operators

use super::update_expression::UpdateExpression;
use crate::{
    syntax::{
        ast::{keyword::Keyword, node::Node, op::UnaryOp, punc::Punctuator, token::TokenKind},
        parser::{AllowAwait, AllowYield, Cursor, ParseError, ParseResult, TokenParser},
    },
    Interner,
};

/// Parses a unary expression.
///
/// More information:
///  - [MDN documentation][mdn]
///  - [ECMAScript specification][spec]
///
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Expressions_and_Operators#Unary
/// [spec]: https://tc39.es/ecma262/#prod-UnaryExpression
#[derive(Debug, Clone, Copy)]
pub(super) struct UnaryExpression {
    allow_yield: AllowYield,
    allow_await: AllowAwait,
}

impl UnaryExpression {
    /// Creates a new `UnaryExpression` parser.
    pub(super) fn new<Y, A>(allow_yield: Y, allow_await: A) -> Self
    where
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
    {
        Self {
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
        }
    }
}

impl TokenParser for UnaryExpression {
    type Output = Node;

    fn parse(self, cursor: &mut Cursor<'_>, interner: &mut Interner) -> ParseResult {
        let tok = cursor.next().ok_or(ParseError::AbruptEnd)?;
        match tok.kind {
            TokenKind::Keyword(Keyword::Delete) => Ok(Node::unary_op(
                UnaryOp::Delete,
                self.parse(cursor, interner)?,
            )),
            TokenKind::Keyword(Keyword::Void) => {
                Ok(Node::unary_op(UnaryOp::Void, self.parse(cursor, interner)?))
            }
            TokenKind::Keyword(Keyword::TypeOf) => Ok(Node::unary_op(
                UnaryOp::TypeOf,
                self.parse(cursor, interner)?,
            )),
            TokenKind::Punctuator(Punctuator::Add) => {
                Ok(Node::unary_op(UnaryOp::Plus, self.parse(cursor, interner)?))
            }
            TokenKind::Punctuator(Punctuator::Sub) => Ok(Node::unary_op(
                UnaryOp::Minus,
                self.parse(cursor, interner)?,
            )),
            TokenKind::Punctuator(Punctuator::Neg) => Ok(Node::unary_op(
                UnaryOp::Tilde,
                self.parse(cursor, interner)?,
            )),
            TokenKind::Punctuator(Punctuator::Not) => {
                Ok(Node::unary_op(UnaryOp::Not, self.parse(cursor, interner)?))
            }
            _ => {
                cursor.back();
                UpdateExpression::new(self.allow_yield, self.allow_await).parse(cursor, interner)
            }
        }
    }
}
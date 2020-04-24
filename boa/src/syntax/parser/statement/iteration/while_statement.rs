use crate::{
    syntax::{
        ast::{keyword::Keyword, node::Node, punc::Punctuator},
        parser::{
            statement::Statement, AllowAwait, AllowReturn, AllowYield, Cursor, Expression,
            ParseResult, TokenParser,
        },
    },
    Interner,
};

/// While statement parsing
///
/// More information:
///  - [MDN documentation][mdn]
///  - [ECMAScript specification][spec]
///
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/while
/// [spec]: https://tc39.es/ecma262/#sec-while-statement
#[derive(Debug, Clone, Copy)]
pub(in crate::syntax::parser::statement) struct WhileStatement {
    allow_yield: AllowYield,
    allow_await: AllowAwait,
    allow_return: AllowReturn,
}

impl WhileStatement {
    /// Creates a new `WhileStatement` parser.
    pub(in crate::syntax::parser::statement) fn new<Y, A, R>(
        allow_yield: Y,
        allow_await: A,
        allow_return: R,
    ) -> Self
    where
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
        R: Into<AllowReturn>,
    {
        Self {
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
            allow_return: allow_return.into(),
        }
    }
}

impl TokenParser for WhileStatement {
    type Output = Node;

    fn parse(self, cursor: &mut Cursor<'_>, interner: &mut Interner) -> ParseResult {
        cursor.expect(Keyword::While, "while statement", interner)?;
        cursor.expect(Punctuator::OpenParen, "while statement", interner)?;

        let cond =
            Expression::new(true, self.allow_yield, self.allow_await).parse(cursor, interner)?;

        cursor.expect(Punctuator::CloseParen, "while statement", interner)?;

        let body = Statement::new(self.allow_yield, self.allow_await, self.allow_return)
            .parse(cursor, interner)?;

        Ok(Node::while_loop(cond, body))
    }
}
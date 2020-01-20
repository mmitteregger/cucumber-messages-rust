//! Pickle types that were previous part of the Gherkin library.
//!
//! A [`Pickle`] represents a template for a `TestCase`. It is typically derived
//! from another format, such as [`GherkinDocument`].
//! In the future a [`Pickle`] may be derived from other formats such as Markdown or
//! Excel files.
//!
//! By making [`Pickle`] the main data structure Cucumber uses for execution, the
//! implementation of Cucumber itself becomes simpler, as it doesn't have to deal
//! with the complex structure of a [`GherkinDocument`].
//!
//! Each [`Step`] of a [`Pickle`] is matched with a [`StepDefinition`] to create a [`TestCase`].
//!
//! [`Pickle`]: ./struct.Pickle.html
//! [`Step`]: ./struct.Step.html
//! [`GherkinDocument`]: ../ast/struct.GherkinDocument.html
//! [`StepDefinition`]: ../command/struct.StepDefinition.html
//! [`TestCase`]: ../test/struct.TestCase.html

pub use crate::proto::generated::pickle::PickleStep as Step;
pub use crate::proto::generated::pickle::PickleTag as Tag;
pub use crate::proto::generated::pickle_step_argument::pickle_table::pickle_table_row::PickleTableCell as TableCell;
pub use crate::proto::generated::pickle_step_argument::pickle_table::PickleTableRow as TableRow;
pub use crate::proto::generated::pickle_step_argument::Message as ArgumentMessage;
pub use crate::proto::generated::pickle_step_argument::PickleDocString as DocString;
pub use crate::proto::generated::pickle_step_argument::PickleTable as Table;
pub use crate::proto::generated::Pickle;
pub use crate::proto::generated::PickleStepArgument as Argument;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<Pickle>();
        assert_sync::<Pickle>();
    }
}

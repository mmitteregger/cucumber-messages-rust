//! AST types that were previous part of the Gherkin library.
//!
//! Cucumber implementations should **not** depend on [`GherkinDocument`] or any of its
//! children for execution - use [`Pickle`] instead.
//!
//! The only consumers of [`GherkinDocument`] should only be formatters that produce
//! "rich" output, resembling the original Gherkin document.
//!
//! [`GherkinDocument`]: ./struct.GherkinDocument.html
//! [`Pickle`]: ../pickle/struct.Pickle.html

pub use crate::proto::generated::gherkin_document::feature::feature_child::rule_child::Value as RuleChildValue;
pub use crate::proto::generated::gherkin_document::feature::feature_child::Rule;
pub use crate::proto::generated::gherkin_document::feature::feature_child::RuleChild;
pub use crate::proto::generated::gherkin_document::feature::feature_child::Value as FeatureChildValue;
pub use crate::proto::generated::gherkin_document::feature::scenario::Examples;
pub use crate::proto::generated::gherkin_document::feature::step::Argument;
pub use crate::proto::generated::gherkin_document::feature::step::DataTable;
pub use crate::proto::generated::gherkin_document::feature::step::DocString;
pub use crate::proto::generated::gherkin_document::feature::table_row::TableCell;
pub use crate::proto::generated::gherkin_document::feature::Background;
pub use crate::proto::generated::gherkin_document::feature::FeatureChild;
pub use crate::proto::generated::gherkin_document::feature::Scenario;
pub use crate::proto::generated::gherkin_document::feature::Step;
pub use crate::proto::generated::gherkin_document::feature::TableRow;
pub use crate::proto::generated::gherkin_document::feature::Tag;
pub use crate::proto::generated::gherkin_document::Comment;
pub use crate::proto::generated::gherkin_document::Feature;
pub use crate::proto::generated::GherkinDocument;
pub use crate::proto::generated::Location;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send<T: Send>() {}

    fn assert_sync<T: Sync>() {}

    #[test]
    fn test_send_sync() {
        assert_send::<GherkinDocument>();
        assert_sync::<GherkinDocument>();
    }
}

//! TestCase, TestStep and related types for test executions.
//!
//! A [`TestCase`] is created by matching a [`Pickle`] with a [`StepDefinition`].
//! It contains a sequence of [`TestStep`]s.
//!
//! [`TestCase`]: ./struct.TestCase.html
//! [`TestStep`]: ./struct.TestStep.html
//! [`Pickle`]: ../pickle/struct.Pickle.html
//! [`StepDefinition`]: ../command/struct.StepDefinition.html

pub use crate::proto::generated::step_match_argument::Group as StepMatchArgumentGroup;
pub use crate::proto::generated::test_case::test_step::StepMatchArgumentsList;
pub use crate::proto::generated::test_case::TestStep;
pub use crate::proto::generated::StepMatchArgument;
pub use crate::proto::generated::TestCase;

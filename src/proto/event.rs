//! Cucumber events.

// Publicly export deprecated types, but ignore them during this library build.
#![allow(deprecated)]

pub use crate::proto::generated::test_step_finished::test_step_result::Status as TestResultStatus;
pub use crate::proto::generated::test_step_finished::TestStepResult;
pub use crate::proto::generated::TestCaseFinished;
pub use crate::proto::generated::TestCaseStarted;
pub use crate::proto::generated::TestRunFinished;
pub use crate::proto::generated::TestRunStarted;
pub use crate::proto::generated::TestStepFinished;
pub use crate::proto::generated::TestStepStarted;

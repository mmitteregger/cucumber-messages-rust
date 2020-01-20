//! Cucumber events.

// Publicly export deprecated types, but ignore them during this library build.
#![allow(deprecated)]

pub use crate::proto::generated::test_case_started::Platform;
pub use crate::proto::generated::test_result::Status as TestResultStatus;
pub use crate::proto::generated::TestCaseFinished;
pub use crate::proto::generated::TestCasePrepared;
pub use crate::proto::generated::TestCasePreparedStep;
pub use crate::proto::generated::TestCaseStarted;
pub use crate::proto::generated::TestResult;
pub use crate::proto::generated::TestRunFinished;
pub use crate::proto::generated::TestRunStarted;
pub use crate::proto::generated::TestStepFinished;
pub use crate::proto::generated::TestStepStarted;

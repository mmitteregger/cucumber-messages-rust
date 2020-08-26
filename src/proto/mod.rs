pub use generated::envelope::Message;
pub use generated::Envelope;
pub use generated::ParseError;

pub mod ast;
pub mod attachment;
pub mod command;
pub mod event;
pub mod pickle;
pub mod source;
pub mod test;
pub mod time;

#[cfg(feature = "ndjson")]
mod serde;

/// Contains generated protobuf code by prost! that will be publicly exported
/// via the pub use and pub mod statements above.
/// This gives more control about the structure, import paths and names.
mod generated {
    // Add deprecation warnings to deprecated types,
    // but build and ignore them in this library
    #![allow(deprecated)]
    // The documentation that is generated from the messages.proto file
    // is not in control of this library. It is synced from the cucumber repository.
    // If you wish to add documentation or get a clarification,
    // create an issue or PR in the cucumber repository instead.
    #![allow(missing_docs)]

    include!(concat!(env!("OUT_DIR"), "/io.cucumber.messages.rs"));
}

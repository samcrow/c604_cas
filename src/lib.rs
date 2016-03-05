
//!
//! Types used for interfacing with the CAS controller
//!
//!
//!

///
/// A message number that plugins should use when submitting a message to the CAS controller.
///
/// When the message is sent, the shared data `SHARED_OBJ_MESSAGE` must be set to the CAS message
/// being submitted.
///
/// The CAS controller plugin will assign a key of type u64 to the message and store it in the
/// shared data `SHARED_OBJ_KEY`.
///
pub const MSG_SUBMIT: i32 = 0x0F00FF00;

///
/// A message number that plugins should use when clearing a submitted message to the CAS
/// controller.
///
/// Plugins should store the message's key in the shared data `SHARED_OBJ_KEY`.
///
pub const MSG_REMOVE: i32 = 0x0F00FF01;

///
/// The name of a shared object used for submitting CASMessages
///
pub const SHARED_OBJ_MESSAGE: &'static str = "c604/systems/cas/interface/message";

///
/// The name of a shared object (with type u64) used to send and receive shared object keys
///
pub const SHARED_OBJ_KEY: &'static str = "c604/systems/cas/interface/message_key";

include!(concat!(env!("OUT_DIR"), "/lib.rs"));

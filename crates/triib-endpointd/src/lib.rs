//! triib's talker and listener entities: the daemon that runs them, and
//! the files it shares with the app, the endpoints to run, what each is
//! doing and how loud its streams are.

pub mod config;
pub mod gptp;
pub mod lite;
pub mod meters;
pub mod port;
pub mod profile;
pub mod runtime;
pub mod status;

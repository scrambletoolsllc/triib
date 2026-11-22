//! A device's log and crash dump, for triib and triib-cli as they read
//! them over ATDECC: the log a device keeps in a LOG_OBJECT memory object
//! and the crash dump it keeps in a CRASH_DUMP one (IEEE 1722.1-2021,
//! 7.2.10), both read through Address Access, so neither needs a console
//! cable. A device is asked nothing unless one of them is being read.

pub mod crash;
pub mod log;

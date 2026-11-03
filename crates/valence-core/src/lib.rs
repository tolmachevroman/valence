//! Valence core: fetches molecule data, caches it, and turns source snippets
//! into validated fact cards. Contains no UI code.

pub mod card;
pub mod ghs;
pub mod http;
pub mod prompt;
pub mod select;
pub mod validate;
pub mod verify;

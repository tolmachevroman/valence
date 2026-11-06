//! Valence core: fetches molecule data, caches it, and turns source snippets
//! into validated fact cards. Contains no UI code.

pub mod card;
pub mod error;
pub mod ghs;
pub mod http;
pub mod prompt;
pub mod pubchem;
pub mod pug_view;
pub mod select;
pub mod text;
pub mod validate;
pub mod verify;
pub mod wikipedia;

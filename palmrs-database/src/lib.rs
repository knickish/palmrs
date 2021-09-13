//! Support for reading, and eventually writing, the Palm OS database formats (PRC and PDB)

mod format;
pub mod format_pdb;
pub mod format_prc;
pub mod header;
pub mod record;
pub mod time;

pub use self::{
	format::{DatabaseFormat, PalmDatabase},
	format_pdb::PdbDatabase,
	format_prc::PrcDatabase,
};

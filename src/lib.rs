//! The `elf` crate defines functions for reading ELF executable files. It
//! handles both 32-bit and 64-bit files, using either big-endian or
//! little-endian byte order.
//!
//! Our intent is for this crate to be trustworthy even when applied to
//! untrusted input, making it suitable for use in security-sensitive tasks like
//! intrusion analysis.
//!
//! The `elf` crate's main types are:
//!
//! : `elf::File`
//!   An ELF file: an object file, executable, or core dump. You can create an
//!   `elf::File` instance given an ELF filename. or a block of bytes. Given a
//!   `File` instance, you can inspect the header, and find segments and
//!   sections.
//!
//! : `elf::Section`
//!   A section, as described by an `elf::File`'s section header table entry.
//!
//! : `elf::Segment`
//!   A segment, as described by an `elf::File`'s program header table entry.

use file;
use section;
use segment;

pub use file::File;
pub use section::Section;
pub use segment::Segment;

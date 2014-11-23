//! The `elf::file::File` type represents an ELF file header. This holds some
//! data needed to interpret the file itself, such as its word size, endianness,
//! and where to find its various parts; and some data about the code it
//! contains, such as the processor architecture and variant flags.

use fixed_endian;

// The `elf::file::Ident` type represents the 'ident' portion of the ELF header,
// the first sixteen bytes of the file. These include the file's magic number,
// file format version, word size, and endianness. All values here are single
// bytes, and tell us how to interpret the multi-byte values found later in the
// header and throughout the file.




// // The ELF file header.
// 
// 
// 
// #![feature(macro_rules)]
// 
// use std::num::Int;
// 
// trait ByteOrder {
//     fn get<T: Int>(n: T) -> T;
//     fn set<T: Int>(n: &mut T, v: T);
// }
// 
// pub struct BigEndian;
// pub struct LittleEndian;
// 
// pub impl ByteOrder for BigEndian {
//     fn get<T: Int>(n: T) -> T { Int::from_be(n) }
//     fn set<T: Int>(n: &mut T, v: T) { *n = v.to_be() }
// }
// 
// pub impl ByteOrder for LittleEndian {
//     fn get<T: Int>(n: T) -> T { Int::from_le(n) }
//     fn set<T: Int>(n: &mut T, v: T) { *n = v.to_le() }
// }
// 
// // Length of FileED::ident.
// const EI_NIDENT: uint = 16;
// 
// // The byte-array portion of the ELF header, common to all word sizes and
// // endiannesses.
// #[repr(packed)]
// pub struct Ident {
//     mag0:       u8,
//     mag1:       u8,
//     mag2:       u8,
//     mag3:       u8,
//     class:      u8,
//     data:       u8,
//     version:    u8,
//     pad:        [u8, ..EI_NIDENT - 6]
// }
// 
// // A common view of any type of ELF file header, independent of class (32-bit vs
// // 64-bit), endianness (BigEndian or LittleEndian), etc.
// pub trait File {
//     fn ident(&self) -> &Ident;
// 
//     // Return true if this file's header is a well-formed ELF header. If this
//     // returns false, attempting to use the remaining methods of this trait will
//     // cause the task to panic.
//     fn well_formed(&self) -> bool { self.ident().well_formed() }
// 
//     // Return the class of this ELF file:
//     // - 32 for a 32-bit ELF file
//     // - 64 for a 64-bit ELF file
//     // - 0 any other class.
//     fn class(&self) -> uint {
//         match self.ident().class {
//             ELFCLASS32 => 32u,
//             ELFCLASS64 => 64u,
//             _ => panic!("Bad ELF class")
//         }
//     }
// 
//     // Return BigEndian if this file is big-endian (i.e., a multi-byte value's
//     // bytes appear in memory with more significant bytes before less
//     // significant); return LittleEndian if it is little-endian (i.e., less
//     // significant before more).
//     fn endianness(&self) -> &ByteOrder {
//         match self.ident().data {
//             ELFDATA2LSB => &LittleEndian as &ByteOrder,
//             ELFDATA2MSB => &BigEndian as &ByteOrder,
//             _ => panic!("Bad ELF data representation")
//         }
//     }
// 
//     // The ELF header version numbers, as it appears in the |ident| portion of
//     // the header, and in the e_version field of the header. These are typically
//     // both 1.
//     // fn versions(&self) -> (u8, u32);
// 
//     // Return the object file type: one of the ET_ constants.
//     fn filetype(&self) -> u16;
// 
//     // Processor-specific flags.
//     // fn flags(&self) -> u32;
// 
//     // Return an array of this file's sections.
//     // fn sections<'f>(&'f self) -> &'f [Section];
// }
// 
// pub trait Section {
// }
// 
// // file format version numbers.
// const EV_NONE : u8 = 0;
// const EV_CURRENT : u8 = 1;
// 
// // file classes.
// const ELFCLASSNONE : u8 = 0;
// const ELFCLASS32   : u8 = 1;
// const ELFCLASS64   : u8 = 2;
// 
// // Data encodings.
// const ELFDATANONE : u8 = 0;
// const ELFDATA2LSB : u8 = 1;
// const ELFDATA2MSB : u8 = 2;
// 
// // Object file types.
// const ET_NONE : u16 = 0;
// const ET_REL  : u16 = 1;
// const ET_EXEC : u16 = 2;
// const ET_DYN  : u16 = 3;
// const ET_CORE : u16 = 4;
// 
// impl Ident {
//     fn well_formed(&self) -> bool {
//         (self.mag0 == 0x7f &&
//          self.mag1 == b'E' &&
//          self.mag2 == b'L' &&
//          self.mag3 == b'F' &&
//          (self.class == ELFCLASS32 || self.class == ELFCLASS64) &&
//          (self.data == ELFDATA2LSB || self.data == ELFDATA2MSB) &&
//          self.version == EV_CURRENT)
//     }
// }
// 
// 
// // Declare a type named $struct_name that uses the unsigned type $word_type for
// // its file offsets and addresses, and that interprets multi-byte values
// // according to $endianness.
// macro_rules! declare_file_header(
//     ($struct_name:ident : $word_type:ty, $endianness:ident) => (
//         #[repr(packed)]
//         pub struct $struct_name {
//             ident:      Ident,
//             filetype:   u16,
//             machine:    u16,
//             version:    u32,
//             entry:      $word_type,
//             phoff:      $word_type,
//             shoff:      $word_type,
//             flags:      u32,
//             ehsize:     u16,
//             phentsize:  u16,
//             phnum:      u16,
//             shentsize:  u16,
//             shnum:      u16,
//             shstrndx:   u16,
//         }
// 
//         pub impl File for $struct_name {
//             fn ident(&self) -> &Ident { &self.ident }
//             fn filetype(&self) -> u16 { $endianness.get(self.filetype); }
//         }
//     )
// )
// 
// declare_file_header!(File32LE: u32, LittleEndian)
// declare_file_header!(File32BE: u32, BigEndian)
// declare_file_header!(File64LE: u64, LittleEndian)
// declare_file_header!(File64BE: u64, BigEndian)
// 
// #[test]
// fn it_works() {
// }

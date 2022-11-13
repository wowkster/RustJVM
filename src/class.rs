use crate::bytes::ByteParseable;
use std::{fs::File, io, path::PathBuf};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/**
 * Represents a structure that can be parsed from a file reader
 */
trait Parseable {
    fn parse(f: &mut File) -> io::Result<Self>
    where
        Self: Sized;
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ClassFile {
    pub magic: [u8; 4],
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: Vec<ClassAccessFlags>,
    pub this_class: u16,
    pub super_class: u16,
}

#[allow(non_snake_case, non_upper_case_globals)]
mod ConstantPoolType {
    pub const Class: u8 = 7;
    pub const Fieldref: u8 = 9;
    pub const Methodref: u8 = 10;
    pub const InterfaceMethodref: u8 = 11;
    pub const String: u8 = 8;
    pub const Integer: u8 = 3;
    pub const Float: u8 = 4;
    pub const Long: u8 = 5;
    pub const Double: u8 = 6;
    pub const NameAndType: u8 = 12;
    pub const Utf8: u8 = 1;
    pub const MethodHandle: u8 = 15;
    pub const MethodType: u8 = 16;
    pub const InvokeDynamic: u8 = 18;
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ConstantPoolInfo {
    Class {
        name_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer {
        value: i32,
    },
    Float {
        value: f32,
    },
    Long {
        value: i64,
    },
    Double {
        value: f64,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8 {
        value: String,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum ClassAccessFlags {
    Public = 0x0001,
    Final = 0x0010,
    Super = 0x0020,
    Interface = 0x0200,
    Abstract = 0x0400,
    Synthetic = 0x1000,
    Annotation = 0x2000,
    Enum = 0x4000,
}

impl Parseable for ClassFile {
    fn parse(f: &mut File) -> io::Result<ClassFile> {
        let magic = f.parse_u4_as_bytes()?;
        let minor_version = f.parse_u2()?;
        let major_version = f.parse_u2()?;

        let constant_pool_count = f.parse_u2()?;

        let mut constant_pool: Vec<ConstantPoolInfo> =
            Vec::with_capacity(constant_pool_count as usize);

        for _ in 1..constant_pool_count {
            let entry = ConstantPoolInfo::parse(f)?;
            constant_pool.push(entry);
        }

        let access_flags_byte = f.parse_u2()?;

        let mut access_flags: Vec<ClassAccessFlags> = Vec::new();

        for flag in ClassAccessFlags::iter() {
            if access_flags_byte & flag.clone() as u16 != 0 {
                access_flags.push(flag)
            }
        }

        let this_class = f.parse_u2()?;
        let super_class = f.parse_u2()?;

        let interfaces_count = f.parse_u2()?;

        for _ in 0..interfaces_count {
            todo!("Implement interfaces parsing");
        }

        let fields_count = f.parse_u2()?;

        for _ in 0..fields_count {
            todo!("Implement fields parsing");
        }

        let methods_count = f.parse_u2()?;

        for _ in 0..methods_count {
            todo!("Implement methods parsing");
        }

        let attributes_count = f.parse_u2()?;

        for _ in 0..attributes_count {
            todo!("Implement attribute parsing");
        }

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
        })
    }
}

impl ClassFile {
    pub fn get_super_class_name(&self) -> &String {
        self.constant_pool.get_class_name_from_index(self.super_class).expect("Could not find name of super class")
    }

    pub fn get_this_class_name(&self) -> &String {
        self.constant_pool.get_class_name_from_index(self.this_class).expect("Could not find name of this class")
    }
}

impl Parseable for ConstantPoolInfo {
    fn parse(f: &mut File) -> io::Result<ConstantPoolInfo> {
        let tag = f.parse_u1()?;

        let info = match tag {
            ConstantPoolType::Class => ConstantPoolInfo::Class {
                name_index: f.parse_u2()?,
            },
            ConstantPoolType::Fieldref => ConstantPoolInfo::Fieldref {
                class_index: f.parse_u2()?,
                name_and_type_index: f.parse_u2()?,
            },
            ConstantPoolType::Methodref => ConstantPoolInfo::Methodref {
                class_index: f.parse_u2()?,
                name_and_type_index: f.parse_u2()?,
            },
            ConstantPoolType::InterfaceMethodref => ConstantPoolInfo::InterfaceMethodref {
                class_index: f.parse_u2()?,
                name_and_type_index: f.parse_u2()?,
            },
            ConstantPoolType::String => ConstantPoolInfo::String {
                string_index: f.parse_u2()?,
            },
            ConstantPoolType::Integer => ConstantPoolInfo::Integer {
                value: f.parse_u4_as_i32()?,
            },
            ConstantPoolType::Float => ConstantPoolInfo::Float {
                value: f.parse_u4_as_f32()?,
            },
            ConstantPoolType::Long => ConstantPoolInfo::Long {
                value: f.parse_u8_as_i64()?,
            },
            ConstantPoolType::Double => ConstantPoolInfo::Double {
                value: f.parse_u8_as_f64()?,
            },
            ConstantPoolType::NameAndType => ConstantPoolInfo::NameAndType {
                name_index: f.parse_u2()?,
                descriptor_index: f.parse_u2()?,
            },
            ConstantPoolType::Utf8 => {
                let length = f.parse_u2()?;

                ConstantPoolInfo::Utf8 {
                    value: f.parse_utf8(length)?,
                }
            }
            ConstantPoolType::MethodHandle => ConstantPoolInfo::MethodHandle {
                reference_kind: f.parse_u1()?,
                reference_index: f.parse_u2()?,
            },
            ConstantPoolType::MethodType => ConstantPoolInfo::MethodType {
                descriptor_index: f.parse_u2()?,
            },
            ConstantPoolType::InvokeDynamic => ConstantPoolInfo::InvokeDynamic {
                bootstrap_method_attr_index: f.parse_u2()?,
                name_and_type_index: f.parse_u2()?,
            },
            _ => {
                eprintln!("Unexpected constant pool type {tag}!");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unexpected constant pool type while parsing class file!",
                ));
            }
        };

        Ok(info)
    }
}

pub trait ConstantPool {
    fn get_value(&self, index: u16) -> &ConstantPoolInfo;
    fn get_class_name_from_index(&self, index: u16) -> Result<&String, ()>;
}

/**
* Allows the costant pool vector to be indexed at 1 instead of 0
*/
impl ConstantPool for Vec<ConstantPoolInfo> {
    fn get_value(&self, index: u16) -> &ConstantPoolInfo {
        let val = self.get(index as usize - 1);
        val.expect(&format!("Illegal index {index} into constant pool!"))
    }

    fn get_class_name_from_index(&self, index: u16) -> Result<&String, ()> {
        let class = self.get_value(index);
        let ConstantPoolInfo::Class { name_index } = class else { return Err(()); };

        let name = self.get_value(*name_index);
        let ConstantPoolInfo::Utf8 { value } = name else { return Err(()); };

        Ok(&value)
    }
}

pub fn parse_class_file(path: &PathBuf) -> io::Result<ClassFile> {
    let mut f = File::open(path)?;

    ClassFile::parse(&mut f)
}

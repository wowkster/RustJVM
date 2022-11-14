use crate::bytes::ByteParsable;
use std::{
    fs::File,
    io::{self, Cursor, Read},
    path::PathBuf,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/**
 * Represents a structure that can be parsed from a file reader
 */
trait Parsable {
    fn parse(f: &mut dyn Read) -> io::Result<Self>
    where
        Self: Sized;
}

/**
 * Represents a structure that can be parsed from a file reader and a class constant poll context
 */
trait ClassParsable {
    fn parse(constant_pool: &dyn ConstantPool, f: &mut dyn Read) -> io::Result<Self>
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
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
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

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum MethodAccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    VarArgs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: Vec<MethodAccessFlags>,
    pub name_index: u16,
    pub name: String,
    pub descriptor_index: u16,
    pub descriptor: String,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_name: String,
    pub attribute: AttributeKind,
}

#[derive(Debug)]
pub enum AttributeKind {
    ConstantValue {
        constant_value_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<Exception>,
        attributes: Vec<AttributeInfo>,
    },
    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile {
        source_file_index: u16,
        source_file_value: String,
    },
    SourceDebugExtension,
    LineNumberTable {
        line_number_table: Vec<LineNumber>,
    },
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    Other {
        bytes: Vec<u8>,
    },
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Exception {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl Parsable for Exception {
    fn parse(mut f: &mut dyn Read) -> io::Result<Self>
    where
        Self: Sized,
    {
        Ok(Exception {
            start_pc: f.parse_u2()?,
            end_pc: f.parse_u2()?,
            handler_pc: f.parse_u2()?,
            catch_type: f.parse_u2()?,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LineNumber {
    start_pc: u16,
    line_number: u16,
}

impl Parsable for LineNumber {
    fn parse(mut f: &mut dyn Read) -> io::Result<Self>
    where
        Self: Sized,
    {
        Ok(LineNumber {
            start_pc: f.parse_u2()?,
            line_number: f.parse_u2()?,
        })
    }
}

impl ClassParsable for AttributeInfo {
    fn parse(constant_pool: &dyn ConstantPool, mut f: &mut dyn Read) -> io::Result<AttributeInfo> {
        let attribute_name_index = f.parse_u2()?;

        let attribute_name = constant_pool
            .get_utf8_from_index(attribute_name_index)
            .expect("Expected value at attribute_name_index to be utf-8")
            .clone();

        println!("==== Parsing attribute: {attribute_name}");

        let attribute_length = f.parse_u4()?;

        let bytes = f.parse_n_bytes(attribute_length as usize)?;
        let mut attribute_bytes = Cursor::new(&bytes);

        let attribute: AttributeKind = match attribute_name.as_str() {
            "ConstantValue" => AttributeKind::ConstantValue {
                constant_value_index: attribute_bytes.parse_u2()?,
            },
            "Code" => {
                let max_stack = attribute_bytes.parse_u2()?;
                let max_locals = attribute_bytes.parse_u2()?;

                let code_length = attribute_bytes.parse_u4()?;

                println!("code_length = {code_length}");

                println!("attribute_bytes = {attribute_bytes:?}");

                let code = attribute_bytes.parse_n_bytes(code_length as usize)?;

                let exception_table_length = attribute_bytes.parse_u2()?;

                let mut exception_table = Vec::with_capacity(exception_table_length as usize);

                for _ in 0..exception_table_length {
                    exception_table.push(Exception::parse(&mut attribute_bytes)?);
                }

                let attributes_count = attribute_bytes.parse_u2()?;

                let mut attributes: Vec<AttributeInfo> =
                    Vec::with_capacity(attributes_count as usize);

                for _ in 0..attributes_count {
                    attributes.push(AttributeInfo::parse(constant_pool, &mut attribute_bytes)?);
                }

                AttributeKind::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                }
            }
            "SourceFile" => {
                let source_file_index = attribute_bytes.parse_u2()?;

                AttributeKind::SourceFile {
                    source_file_index,
                    source_file_value: constant_pool
                        .get_utf8_from_index(source_file_index)
                        .expect("Expected source file name to be utf-8")
                        .clone(),
                }
            }
            "LineNumberTable" => {
                let line_number_table_length = attribute_bytes.parse_u2()?;

                let mut line_number_table: Vec<LineNumber> =
                    Vec::with_capacity(line_number_table_length as usize);

                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumber::parse(&mut attribute_bytes)?);
                }

                AttributeKind::LineNumberTable { line_number_table }
            }
            _ => {
                eprintln!("[WARN] Got unexpected attribute kind with name: {attribute_name}");
                AttributeKind::Other {
                    bytes: bytes.to_vec(),
                }
            }
        };

        Ok(AttributeInfo {
            attribute_name_index,
            attribute_name,
            attribute,
        })
    }
}

impl ClassParsable for MethodInfo {
    fn parse(constant_pool: &dyn ConstantPool, mut f: &mut dyn Read) -> io::Result<MethodInfo> {
        let access_flags_byte = f.parse_u2()?;

        let mut access_flags: Vec<MethodAccessFlags> = Vec::new();

        for flag in MethodAccessFlags::iter() {
            if access_flags_byte & flag.clone() as u16 != 0 {
                access_flags.push(flag)
            }
        }

        let name_index = f.parse_u2()?;
        let name = constant_pool
            .get_utf8_from_index(name_index)
            .expect("Expected value at attribute_name_index to be utf-8")
            .clone();

        let descriptor_index = f.parse_u2()?;
        let descriptor = constant_pool
            .get_utf8_from_index(descriptor_index)
            .expect("Expected value at descriptor to be utf-8")
            .clone();

        let attributes_count = f.parse_u2()?;

        let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::parse(constant_pool, f)?);
        }

        Ok(MethodInfo {
            access_flags,
            name_index,
            name,
            descriptor_index,
            descriptor,
            attributes,
        })
    }
}

impl Parsable for ClassFile {
    fn parse(mut f: &mut dyn Read) -> io::Result<ClassFile> {
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

        let mut methods: Vec<MethodInfo> = Vec::with_capacity(methods_count as usize);

        for _ in 0..methods_count {
            methods.push(MethodInfo::parse(&constant_pool, f)?);
        }

        let attributes_count = f.parse_u2()?;

        let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(methods_count as usize);

        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::parse(&constant_pool, f)?);
        }

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            methods,
            attributes,
        })
    }
}

impl ClassFile {
    pub fn get_super_class_name(&self) -> &String {
        self.constant_pool
            .get_class_name_from_index(self.super_class)
            .expect("Could not find name of super class")
    }

    pub fn get_this_class_name(&self) -> &String {
        self.constant_pool
            .get_class_name_from_index(self.this_class)
            .expect("Could not find name of this class")
    }
}

impl Parsable for ConstantPoolInfo {
    fn parse(mut f: &mut dyn Read) -> io::Result<ConstantPoolInfo> {
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
    fn get_utf8_from_index(&self, index: u16) -> Result<&String, ()>;
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

    fn get_utf8_from_index(&self, index: u16) -> Result<&String, ()> {
        let utf8 = self.get_value(index);
        let ConstantPoolInfo::Utf8 { value } = utf8 else { return Err(()); };

        Ok(&value)
    }
}

pub fn parse_class_file(path: &PathBuf) -> io::Result<ClassFile> {
    let mut f = File::open(path)?;

    ClassFile::parse(&mut f)
}

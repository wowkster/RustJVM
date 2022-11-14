use core::panic;
use std::io::Cursor;

use crate::{
    bytes::ByteParsable,
    class::{AttributeKind, ClassFile, ConstantPool, ConstantPoolInfo},
};

#[allow(non_snake_case, non_upper_case_globals, dead_code)]
mod OpCodeType {
    pub const getstatic: u8 = 0xb2;
    pub const invokevirtual: u8 = 0xb6;
    pub const ldc: u8 = 0x12;
}

#[allow(dead_code)]
pub struct Instance {
    class_name: String,
}

pub enum OperandStackEntry {
    ClassInstance(Instance),
    Int(i32),
    Float(f32),
    String(String),
}

pub fn run_main(class: &ClassFile) -> std::io::Result<()> {
    let Some(main) = class.get_main_method() else {
        panic!("No main method found in class {}", class.get_this_class_name())
    };

    let code_attribute = main.get_code();

    let AttributeKind::Code { max_stack: _, max_locals: _, code, exception_table: _, attributes: _ } = &code_attribute.attribute else {
        panic!("Code attribute")
    };

    let mut byte_code = Cursor::new(code);
    let len = byte_code.get_ref().len();

    let mut operand_stack: Vec<OperandStackEntry> = Vec::new();

    while (byte_code.position() as usize) < len - 1 {
        let instruction = byte_code.parse_u1()?;

        // println!("Read byte: 0x{:02x?}", instruction);

        match instruction {
            OpCodeType::getstatic => {
                let field_ref_index = byte_code.parse_u2()?;

                let field_ref = class.constant_pool.get_value(field_ref_index);

                let ConstantPoolInfo::Fieldref { class_index, name_and_type_index } = field_ref else {
                    panic!("Expected field ref to be of type Fieldref")
                };

                let _field_class = class
                    .constant_pool
                    .get_class_name_from_index(*class_index)
                    .expect("Expected field to have valid class index");

                let name_and_type = class
                    .constant_pool
                    .get_name_and_type(*name_and_type_index)
                    .expect("Expected field to have valid name_and_type index");

                operand_stack.push(OperandStackEntry::ClassInstance(Instance {
                    class_name: name_and_type.1.clone(),
                }));

                // println!("Class = {field_class}");
                // println!("Name and Type = {name_and_type:?}");
            }
            OpCodeType::ldc => {
                let constant_index = byte_code.parse_u1()?;

                match class.constant_pool.get_value(constant_index as u16) {
                    ConstantPoolInfo::String { string_index } => {
                        operand_stack.push(OperandStackEntry::String(
                            class
                                .constant_pool
                                .get_utf8_from_index(*string_index)
                                .expect("Expected string_index to be utf-8")
                                .clone(),
                        ))
                    }
                    _ => panic!("Unexpected constant type"),
                }
            }
            OpCodeType::invokevirtual => {
                let method_index = byte_code.parse_u2()?;

                let method_ref = class.constant_pool.get_value(method_index);

                let ConstantPoolInfo::Methodref { class_index, name_and_type_index } = method_ref else {
                        panic!("Expected field ref to be of type Fieldref")
                    };

                let field_class = class
                    .constant_pool
                    .get_class_name_from_index(*class_index)
                    .expect("Expected field to have valid class index");

                let name_and_type = class
                    .constant_pool
                    .get_name_and_type(*name_and_type_index)
                    .expect("Expected field to have valid name_and_type index");

                // println!("Class = {field_class}");
                // println!("Name and Type = {name_and_type:?}");
                
                if field_class == "java/io/PrintStream"
                && name_and_type.0 == "println"
                && name_and_type.1 == "(Ljava/lang/String;)V"
                {
                    let OperandStackEntry::String(string) = operand_stack.pop().unwrap() else {panic!("Expected operand stack to contain string")};
                    let _instance = operand_stack.pop();

                    println!("{string}")
                }
            }
            _ => {
                todo!("Instruction 0x{instruction:02x?} is not yet implemented")
            }
        }
    }

    Ok(())
}

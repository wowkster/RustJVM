use std::{env, path::PathBuf};

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_name = args.get(1).expect("Exected file name argument!");

    let class = rust_jvm::class::parse_class_file(&PathBuf::from(file_name))
        .expect("Could not parse class file");

    println!("{:#?}", class);

    assert_eq!(
        &class.magic, b"\xCA\xFE\xBA\xBE",
        "Class magic should be [CA, FE, BA, BE], but got: {:02X?}",
        &class.magic
    );

    assert_eq!(class.get_super_class_name(), "java/lang/Object");
    assert_eq!(class.get_this_class_name(), "Main");
}

use std::path::Path;

use crate::{
    add_bool, define_module,
    maggi_obj::{MaggiObj, ObjectTypes},
    set_bool,
};

use pelite::{
    pe64::{Pe, PeFile},
    FileMap,
};

fn parse_headers<P: AsRef<Path> + ?Sized>(obj: &mut MaggiObj, path: &P) {
    let path = path.as_ref();
    if let Ok(map) = FileMap::open(path) {
        if let Ok(file) = PeFile::from_bytes(&map) {
            set_bool!(obj, "is_pe", true);
            println!("Load addr: {}", file.optional_header().ImageBase);
        }
    }
}

fn parse_imports() {}

fn parse_exports() {}

//for pe module we can scan executing processes and files. we ned universal functions for this work
pub fn module_init() -> Option<MaggiObj> {
    let mut obj = define_module!("pe");

    add_bool!(obj, "is_32bit");
    add_bool!(obj, "is_64bit");
    add_bool!(obj, "is_pe");
    add_bool!(obj, "is_dll");

    parse_headers(
        &mut obj,
        "C:\\Users\\User\\Documents\\GitHub\\rust-commander\\target\\debug\\rust-commander.exe",
    );
    parse_imports();
    parse_exports();

    println!("Is PE:{:?}", obj.find_prop("is_pe").unwrap().get_val());

    Some(obj)
}

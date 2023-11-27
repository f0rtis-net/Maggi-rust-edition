use std::{collections::HashMap, f32};

#[derive(Debug)]
pub enum ObjectTypes {
    INTEGER(i32),
    FLOAT(f32),
    STRING,
    BOOLEAN(bool),
}

pub struct ObjSlot {
    name: &'static str,
    val_n_type: ObjectTypes,
}

impl ObjSlot {
    pub fn get_name(&self) -> &'static str {
        self.name
    }
    pub fn get_val(&self) -> &ObjectTypes {
        &self.val_n_type
    }

    pub fn set_val(&mut self, new_val: ObjectTypes) {
        self.val_n_type = new_val;
    }
}

pub struct MaggiObj {
    objects: HashMap<String, ObjSlot>,
    prefix: &'static str,
}

impl MaggiObj {
    pub fn new(prefix: &'static str) -> Self {
        Self {
            objects: HashMap::new(),
            prefix,
        }
    }

    pub fn find_prop(&mut self, pname: &'static str) -> Option<&mut ObjSlot> {
        self.objects.get_mut(pname)
    }

    pub fn add_prop(&mut self, name: &'static str, val_n_type: ObjectTypes) {
        self.objects
            .insert(name.to_string(), ObjSlot { name, val_n_type });
    }
}

#[macro_export]
macro_rules! set_bool {
    ($obj:expr, $name:expr, $value:expr) => {
        $obj.find_prop($name)
            .unwrap()
            .set_val(ObjectTypes::BOOLEAN($value));
    };
}

#[macro_export]
macro_rules! add_bool {
    ($obj:ident, $name:expr) => {
        $obj.add_prop($name, ObjectTypes::BOOLEAN(false));
    };
}

#[macro_export]
macro_rules! define_module {
    ($name:expr) => {{
        let obj = MaggiObj::new($name);
        obj
    }};
}

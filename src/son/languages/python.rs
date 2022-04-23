use std::collections::HashMap;
use std::io::prelude::Write;
use crate::son;

pub fn build(file_path: &str, class_name: &str, map: HashMap<String, son::Types>){
    
    let mut f = std::fs::File::create(file_path).unwrap();

    f.write(format!("class {}:\n", class_name).as_bytes()).unwrap();
    for (name, typ) in map.iter(){
        f.write(format!("    {}:", name).as_bytes()).unwrap();
        match typ {
            son::Types::Int     => f.write(b"int").unwrap(),
            son::Types::Long    => f.write(b"int").unwrap(),
            son::Types::Uint    => f.write(b"int").unwrap(),
            son::Types::Ulong   => f.write(b"int").unwrap(),
            son::Types::Bool    => f.write(b"bool").unwrap(),
            son::Types::F32     => f.write(b"float").unwrap(),
            son::Types::F64     => f.write(b"float").unwrap(),
            son::Types::Char    => f.write(b"str").unwrap(),
            son::Types::Str     => f.write(b"str").unwrap(),
            son::Types::Vec     => todo!(),
            son::Types::Arr(_)  => todo!(),
            son::Types::Obj(_)  => todo!()
        };
        f.write(b"\n").unwrap();
    }
    
}
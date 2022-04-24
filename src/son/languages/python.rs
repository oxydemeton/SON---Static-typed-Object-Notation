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
    
    //From String constant part of the function
    f.write(b"\n    def from_string(self, txt: str):\n        lines = txt.split(\"\\n\")\n        for i in range(len(lines)):\n            lines[i] = lines[i].strip()\n        for l in lines:\n            (name, value) = l.split(\"=\")\n            name = name.strip()\n            value = value.strip()\n").unwrap();    

    f.write(b"            ").unwrap();
    for (name, typ) in map.iter(){
        f.write(format!("if name == \"{}\":\n", name).as_bytes()).unwrap();
        match typ{
            son::Types::Int     => f.write(format!("                self.{} = int(value)\n", name).as_bytes()).unwrap(),
            son::Types::Long    => f.write(format!("                self.{} = int(value)\n", name).as_bytes()).unwrap(),
            son::Types::Uint    => f.write(format!("                self.{} = int(value)\n", name).as_bytes()).unwrap(),
            son::Types::Ulong   => f.write(format!("                self.{} = int(value)\n", name).as_bytes()).unwrap(),
            son::Types::Bool    => f.write(format!("                self.{} = bool(value)\n", name).as_bytes()).unwrap(),
            son::Types::F32     => f.write(format!("                self.{} = float(value)\n", name).as_bytes()).unwrap(),
            son::Types::F64     => f.write(format!("                self.{} = float(value)\n", name).as_bytes()).unwrap(),
            son::Types::Char    => f.write(format!("                self.{} = value.replace(\"\\\'\", \"\")\n", name).as_bytes()).unwrap(),
            son::Types::Str     => f.write(format!("                self.{} = value.replace(\"\\\"\", \"\")\n", name).as_bytes()).unwrap(),
            son::Types::Vec     => todo!(),
            son::Types::Arr(_)  => todo!(),
            son::Types::Obj(_)  => todo!()
        };
        f.write(b"            el").unwrap();
    }
    f.write(b"se:\n                raise Exception(\"Unknown parameter: \" + name)\n\n").unwrap();

    //to_string function
    f.write(b"    def to_string(self):\n        out  = \"\"\n").unwrap();
    for (name, _) in map.iter(){
        f.write(format!("        out += \"{}=\" + str(self.{})\n", name, name).as_bytes()).unwrap();
    }
    f.write(b"        return out\n\n").unwrap();

    //get_def function
    f.write(b"    def get_def(self):\n        out  = \"\"\n").unwrap();
    for (name, typ) in map.iter(){
        f.write(format!("        out += \"{:?} {}\"\n", typ, name).replace("Str", "String").as_bytes()).unwrap();
    }
    f.write(b"        return out\n\n").unwrap();

    //Constructor
    f.write(b"    def __init__(self, txt: str):\n        self.from_string(txt)\n").unwrap();
}
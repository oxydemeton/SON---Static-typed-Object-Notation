use std::collections::HashMap;
use crate::son;

pub fn read_file_lines(path: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    contents.lines().map(|s| s.to_string()).collect()
}

fn line_to_type(typ_str: &String) -> Option<son::Types>{
    let mut typ: Option<son::Types> = None;

    if typ_str == "int"{
        typ = Some(son::Types::Int);
    }else if typ_str == "long"{
        typ = Some(son::Types::Long);
    }else if typ_str == "uint"{
        typ = Some(son::Types::Uint);
    }else if typ_str == "ulong"{
        typ = Some(son::Types::Ulong);
    }else if typ_str == "bool"{
        typ = Some(son::Types::Bool);
    }else if typ_str == "f32"{
        typ = Some(son::Types::F32);
    }else if typ_str == "f64"{
        typ = Some(son::Types::F64);
    }else if typ_str == "char"{
        typ = Some(son::Types::Char);
    }else if typ_str == "str"{
        typ = Some(son::Types::Str);
    }else if typ_str.starts_with("vec:"){
        let vec_str = &typ_str[4..].to_string();
        if line_to_type(vec_str).is_some(){
            typ = Some(son::Types::Vec(Box::new(line_to_type(vec_str).unwrap())));
        }else{
            println!("Error: Invalid type in vec: {}", vec_str);
        }
    }

    typ
}


pub fn lines_to_hashmap(lines: Vec<String>) -> HashMap<String, son::Types> {
    
    let mut map: HashMap<String, son::Types> = HashMap::new();
    for mut line in lines {
        line = String::from(line.trim());
        if line.contains(" ") {
            let mut words = line.split(" ");
            let typ_str: String = words.next().unwrap().to_lowercase();
            let typ_str = typ_str.replace(" ", "");
            let key: &str = words.next().unwrap();
            let key = key.replace(" ", "");
            let typ: Option<son::Types> = line_to_type(&typ_str);
            
            match typ{
                None => println!("Error: Invalid type given."),
                Some(v) => { match map.insert(key, v){
                    None=>{},
                    Some(_) => println!("Error: Value with the name \"{}\" is already used", typ_str)
                }}
            };
            
        }
    }
    map
}
pub mod son {
    mod file_processing;
    use std::collections::HashMap;
    
    #[derive(Debug, Clone)]
    pub enum Types{
        Int,
        Long,
        Uint,
        Ulong,
        Bool,
        F32,
        F64,
        Char,
        Str,
        Vec(Box<Types>),
        Arr(usize),
        Obj(HashMap<String, Types>)
    }

    mod languages;

    
    pub fn process_son_def(file_path: &str, class_name: &str){
        let lines = file_processing::read_file_lines(file_path);
        let map = file_processing::lines_to_hashmap(lines);
        println!("{:?}", map);
        let path = std::env::current_dir().unwrap().join("son_impl");
        std::fs::create_dir_all(&path).unwrap();
        let py_file_name = String::from(path.to_str().unwrap()) + "/" + class_name + ".py";
        languages::python::build(&py_file_name, class_name, map);
    }
}
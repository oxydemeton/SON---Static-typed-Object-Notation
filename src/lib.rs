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

    //function called process_son_def which takes a str and returns a Result of a Map of str to str
    pub fn process_son_def(file_path: &str, class_name: &str){
        //read file lines and call lines_to_hashmap
        let lines = file_processing::read_file_lines(file_path);
        let map = file_processing::lines_to_hashmap(lines);
        println!("{:?}", map);
        let py_file_name = String::from(class_name) + ".py";
        languages::python::build(&py_file_name, class_name, map);
    }
}
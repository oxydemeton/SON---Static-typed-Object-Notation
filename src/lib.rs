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
    pub fn process_son_def(file_path: &str){
        //read file lines and call lines_to_hashmap
        let lines = file_processing::read_file_lines(file_path);
        let map = file_processing::lines_to_hashmap(lines);
        println!("{:?}", map);
        //languages::python::build("example.py", "example_class", map);

    }
}
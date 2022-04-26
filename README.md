# SON - Static typed object Notation
Son is an object Notation wich is similar to json but can be included in different programming languanges.
- [x] `Python`
- [ ] `Rust`
- [ ] `Cpp`
- [ ] `TypeScript`
- [ ] `JavaScript`
Those languanges aren't final and can change any time but it is the goal for now.

## How to use
### Create Definition
Create a new file ending with `.sondef`.
Put your definition into the file:
```
String name
uint age
f32 size
.
.
.
```
First write the type followed by the name of the variable.
Following types are supported.
- [x] `Int` (32bit)
- [x] `Uint` (32bit)
- [x] `Long` (64bit)
- [x] `Ulong` (64bit)
- [x] `Bool`
- [x] `F32` (32bit Float)
- [x] `F64` (64bit Float/Double)
- [x] `Char`
- [x] `String` 
- [ ] `Vec` (Vector/dynamic sized array)
- [ ] `Arr` (fixed sized array)
- [ ] `obj` (not final idea; Subobjects)

### Build implementations
```bash
$ cargo run example.sondef class_name
```
Replace example.son with your definition files name and class_name with your favorite name for the output classes.

### Use the implementations
You can find the implementations inside the son_impl folder for each language.
They include the calass/struct with the values, types and function to read and write string to/from an object.

class example_class:
    str_string:str
    char:str
    integer:int
    def from_string(self, txt: str):
        lines = txt.split("\n")
        for i in range(len(lines)):
            lines[i] = lines[i].strip()
        for l in lines:
            (name, value) = l.split("=")
            name = name.strip()
            value = value.strip()
            if name == "str_string":
                self.str_string = value.replace(" \"", "")
            elif name == "char":
                self.char = value.replace(" \'", "")
            elif name == "integer":
                self.integer = int(value)
            else:
                raise Exception("Unknown parameter: " + name)


if __name__ == "__main__":
    a = example_class()
    a.from_string("str_string = \"Hello World\"\ninteger = 1337\nchar = \"a\"")
    print(a.str_string, a.char, a.integer)

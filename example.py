class example_class:
    str_string:str
    integer:int
    char:str
    def from_string(self, txt: str):
        lines = txt.split("\n")
        for i in range(len(lines)):
            lines[i] = lines[i].strip()
        for l in lines:
            (name, value) = l.split("=")
            name = name.strip()
            value = value.strip()
            if name == "str_string":
                self.str_string = value.replace("\"", "")
            elif name == "integer":
                self.integer = int(value)
            elif name == "char":
                self.char = value.replace("\'", "")
            else:
                raise Exception("Unknown parameter: " + name)

class example_class:
    char:str
    str_string:str
    integer:int
    def from_string(self, txt: str):
        lines = txt.split("\n")
        for i in range(len(lines)):
            lines[i] = lines[i].strip()
        for l in lines:
            (name, value) = l.split("=")
            name = name.strip()
            value = value.strip()
            if name == "char":
                self.char = value.replace("\'", "")
            elif name == "str_string":
                self.str_string = value.replace("\"", "")
            elif name == "integer":
                self.integer = int(value)
            else:
                raise Exception("Unknown parameter: " + name)
    def __init__(self, txt: str):
        self.from_string(txt)

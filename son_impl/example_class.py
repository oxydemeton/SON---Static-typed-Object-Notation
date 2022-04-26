class example_class:
    followers:int
    latest_video_url:str
    name:str
    url:str

    def from_string(self, txt: str):
        lines = txt.split("\n")
        for i in range(len(lines)):
            lines[i] = lines[i].strip()
        for l in lines:
            (name, value) = l.split("=")
            name = name.strip()
            value = value.strip()
            if name == "followers":
                self.followers = int(value)
            elif name == "latest_video_url":
                self.latest_video_url = value.replace("\"", "")
            elif name == "name":
                self.name = value.replace("\"", "")
            elif name == "url":
                self.url = value.replace("\"", "")
            else:
                raise Exception("Unknown parameter: " + name)

    def to_string(self):
        out  = ""
        out += "followers=" + str(self.followers)
        out += "latest_video_url=" + str(self.latest_video_url)
        out += "name=" + str(self.name)
        out += "url=" + str(self.url)
        return out

    def get_def(self):
        out  = ""
        out += "Uint followers"
        out += "Str latest_video_url"
        out += "Str name"
        out += "Str url"
        return out

    def __init__(self, txt: str):
        self.from_string(txt)

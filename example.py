class example_class:
    name:str
    url:str
    followers:int
    latest_video_url:str

    def from_string(self, txt: str):
        lines = txt.split("\n")
        for i in range(len(lines)):
            lines[i] = lines[i].strip()
        for l in lines:
            (name, value) = l.split("=")
            name = name.strip()
            value = value.strip()
            if name == "name":
                self.name = value.replace("\"", "")
            elif name == "url":
                self.url = value.replace("\"", "")
            elif name == "followers":
                self.followers = int(value)
            elif name == "latest_video_url":
                self.latest_video_url = value.replace("\"", "")
            else:
                raise Exception("Unknown parameter: " + name)

    def to_string(self):
        out  = ""
        out += "name=" + str(self.name)
        out += "url=" + str(self.url)
        out += "followers=" + str(self.followers)
        out += "latest_video_url=" + str(self.latest_video_url)
        return out

    def get_def(self):
        out  = ""
        out += "String name"
        out += "String url"
        out += "Uint followers"
        out += "String latest_video_url"
        return out

    def __init__(self, txt: str):
        self.from_string(txt)

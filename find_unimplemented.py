import os

src = os.scandir("./raylib/src/core/")
src_files = []

for file in src:
    if file.is_file(follow_symlinks=True):
        f = open(file.path)
        src_files.append("\n".join(f.readlines()))

d = open("./raylib-sys/raylib/src/raylib.h")
lines = list(filter(lambda f: f.startswith("RLAPI"),d.readlines()))

for line in lines:
    func_name = list(filter(lambda f: "(" in f, line.split(" ")))[0].split("(")[0].replace("*","")
    in_a_file = False
    for file in src_files:
        if "ffi::"+func_name in file:
            in_a_file = True
            break
    if not in_a_file:
        print("- [ ] "+func_name)
import os

# Functions we won't implement.
wont_impl = [
    # We have this implemented in a C file so it's not caught.
    "SetTraceLogCallback",
    # UTF-8 functions
    "GetCodepointNext",
    "GetCodepointPrevious",
    "CodepointToUTF8",
    "LoadUTF8",
    "UnloadUTF8",
    # Text functions
    "TextCopy",
    "TextIsEqual",
    "TextLength",
    "TextFormat",
    "TextSubtext",
    "TextReplace",
    "TextInsert",
    "TextJoin",
    "TextSplit",
    "TextAppend",
    "TextFindIndex",
    "TextToUpper",
    "TextToLower",
    "TextToPascal",
    "TextToSnake",
    "TextToCamel",
    "TextToInteger",
    "TextToFloat",
    # file functions
    "LoadFileData",
	"UnloadFileData",
	"SaveFileData",
	"LoadFileText",
	"UnloadFileText",
	"SaveFileText",
	"FileExists",
	"DirectoryExists",
	"GetFileExtension",
	"GetFileName",
	"GetFileNameWithoutExt",
	"GetDirectoryPath",
	"GetPrevDirectoryPath",
	"GetWorkingDirectory",
	"MakeDirectory",
	"ChangeDirectory",
	"IsFileNameValid",
	"GetFileModTime",
	"ComputeCRC32",
	"ComputeMD5",
	"ComputeSHA1",
    # Misc functions that aren't needed.
    "MemRealloc",
]


def file_find(lib, src, dest, opener):
    print("=====",lib,"=====")

    files = os.scandir(dest)
    src_files = []

    for file in files:  
        if file.is_file(follow_symlinks=True):
            f = open(file.path)
            src_files.append("\n".join(f.readlines()))

    d = open(src)
    lines = list(filter(lambda f: f.startswith(opener),d.readlines()))
    
    for line in lines:
        func_name = list(filter(lambda f: "(" in f, line.split(" ")))[0].split("(")[0].replace("*","")

        in_a_file = False
        for file in src_files:
            if func_name in wont_impl or "ffi::"+func_name in file:
                in_a_file = True
                break
        if not in_a_file:
            print("- [ ] "+func_name)
    print("")

file_find("Raylib","./raylib-sys/raylib/src/raylib.h", "./raylib/src/core/", "RLAPI")
file_find("Raygui","./raylib-sys/binding/raygui.h","./raylib/src/rgui", "    RAYGUIAPI")

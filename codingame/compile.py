import re
from pathlib import Path
import pyperclip
import sys

debug = True
try:
    debug = sys.argv[1] != "build"
except:
    pass

APP = Path(__file__).parent
DIST = APP / 'dist'
SRC = APP / 'src'

files = SRC.glob('*')
template = {}
for file in files:
    template[f"{{{file.name}}}"] = file.read_text(encoding='utf-8')

text: str = template['{root.rs}']
del template['{root.rs}']


def replace(text: str, template: dict):
    res = ""
    for line in text.splitlines():
        if "python:del" in line:
            continue
        if not debug and "python:debug" in line:
            continue
        elif "python:replace" in line:
            text = line.split("python:replace")[1].strip()
            for key in template.keys():
                text = text.replace(key, template[key])
            res += text + "\n"
            continue
        else:
            res += line + "\n"
    return res


def del_comment(text: str):
    res = ""
    for line in text.splitlines():
        if r"//" in line:
            res += line.split(r"//")[0] + "\n"
        else:
            res += line + "\n"
    return res


for key in template.keys():
    template[key] = replace(template[key], template)
    if not debug:
        template[key] = del_comment(template[key])

text = replace(text, template)
if not debug:
    text = del_comment(text)
    text = text.replace("\n", "")
    text = re.sub(r' +', ' ', text)

(DIST / 'main.rs').write_text(text, encoding='utf-8')
print(f"compile: {len(text)} bytes")
pyperclip.copy(text)
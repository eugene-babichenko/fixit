import re

with open("CHANGELOG.md", "r") as file:
    content = file.read()
headings = list(re.finditer(r"^##\s+.+$", content, re.MULTILINE))
start_pos = headings[1].end() + 1
end_pos = headings[2].start()
print(content[start_pos:end_pos].strip())

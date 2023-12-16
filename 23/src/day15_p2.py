import re

with open("../input/2023/day15.txt") as o:
	input = o.read()

def hash(s):
	v = 0
	for c in s:
		v = ((v + ord(c)) * 17) & 0xFF
	return v

map = {}
for x in re.finditer(r"([^,]*)(?:-|=([0-9]))(?:,|$)", input.strip()):
	label = x.group(1)
	focal_len = x.group(2)
	if focal_len is None:
		map.pop(label, None)
	else:
		map[label] = int(focal_len)

lens = [0] * 256
result = 0
for (label, focal_len) in map.items():
	box = hash(label)
	index = lens[box] = lens[box] + 1
	result += focal_len * (box + 1) * index

print(result)
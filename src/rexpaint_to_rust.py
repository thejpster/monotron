#!/usr/bin/python3

import sys
import struct
import codecs

cp437 = codecs.lookup("cp437")

if sys.argv[1] == "-":
	data = sys.stdin.buffer.read()
else:
	with open(sys.argv[1], "rb") as f:
		data = f.read()

print("Got {} bytes".format(len(data)))

(ver, layers, width, height) = struct.unpack("<LLLL", data[0:16])
assert ver == 0xFFFFFFFF, "Bad version"
assert layers == 0x00000001, "Too many layers"

cells = {}

offset = 16
for col in range(0, width):
	for row in range(0, height):
		(char, fr, fg, fb, br, bg, bb) = struct.unpack("<LBBBBBB", data[offset:offset+10])
		# We had to put magenta in at 254 to avoid a palette bug
		if fb == 254:
			fb = 255
		if bb == 254:
			bb = 255
		fg = (fr, fg, fb)
		bg = (br, bg, bb)
		offset += 10
		uni = cp437.decode(bytes([char]))[0]
		cells[(row, col)] = (uni, fg, bg)

rgb_map = {
	(255, 255, 255) : 'A',
	(255, 255, 0) : 'B',
	(255, 0, 255) : 'C',
	(255, 0, 0) : 'D',
	(0, 255, 255) : 'E',
	(0, 255, 0) : 'F',
	(0, 0, 255) : 'G',
	(0, 0, 0) : 'H',
}

current_fg = ''
current_bg = ''
for row in range(0, height):
	sys.stdout.write("write!(context, \"");
	for col in range(0, width):
		(ch, fg, bg) = cells[row, col]
		fg_ch = rgb_map[fg]
		bg_ch = rgb_map[bg].lower()
		if fg_ch != current_fg:
			sys.stdout.write("\\u{{001b}}{}".format(fg_ch))
			current_fg = fg_ch
		if bg_ch != current_bg:
			sys.stdout.write("\\u{{001b}}{}".format(bg_ch))
			current_bg = bg_ch
		if ord(ch) >= 32:
			sys.stdout.write("{}".format(ch))
		else:
			sys.stdout.write("\\u{{{:04x}}}".format(ord(ch)))
	sys.stdout.write("\").unwrap();\n")

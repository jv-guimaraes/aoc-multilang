extends MainLoop

var Password: = load("password.gd")

func _process(_delta):
	return true

func read_and_parse_input(filename: String) -> Array:
	var lines = FileAccess.open(filename, FileAccess.READ).get_as_text().split("\r\n")
	var res = []
	for line in lines:
		var line_split = line.split(" ")
		var range_split = line_split[0].split("-")
		var c = line_split[1][0]
		var text = line_split[2]
		var password = Password.new(int(range_split[0]), int(range_split[1]), c, text)
		res.append(password)
	return res

func _init():
	var passwords = read_and_parse_input("input.txt")

	var valid_count = [0, 0]
	for pw in passwords:
		if pw.is_valid(): valid_count[0] += 1
		if pw.is_valid2(): valid_count[1] += 1
	
	print("Part 1: ", valid_count[0])
	print("Part 2: ", valid_count[1])

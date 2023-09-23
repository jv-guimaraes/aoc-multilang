extends MainLoop

func _process(_delta):
	return true

func read_input(filename: String) -> PackedStringArray:
	return FileAccess.open(filename, FileAccess.READ).get_as_text().split("\r\n")

func count_trees(input: PackedStringArray, right: int, down: int) -> int:
	var width = len(input[0])
	var height = len(input)
	var x = 0; var y = 0
	var count = 0

	while y < height:
		if input[y][x % width] == "#":
			count += 1
		x += right
		y += down

	return count

func _init():
	var input = read_input("input.txt")
	print("Part 1: ", count_trees(input, 3, 1))

	var total = 1
	for slope in [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]:
		total *= count_trees(input, slope[0], slope[1])
	print("Part 2: ", total)
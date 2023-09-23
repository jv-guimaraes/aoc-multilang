extends MainLoop

func parse_input() -> Array:
	var file = FileAccess.open("input.txt", FileAccess.READ)
	var split = file.get_as_text().split("\n")
	split = split.slice(0, len(split) - 1)
	var res = []
	for num in split: res.append(int(num))
	return res
	
func part1(input: Array) :
	for i in range(len(input)):
		for j in range(len(input)):
			if input[i] + input[j] == 2020:
				print("Part 1: ", input[i] * input[j])
				return
				
func part2(input: Array) :
	for i in range(len(input)):
		for j in range(len(input)):
			for k in range(len(input)):
				if input[i] + input[j] + input[k] == 2020:
					print("Part 2: ", input[i] * input[j] * input[k])
					return

func _init():
	var parsed_input := parse_input()
	part1(parsed_input)
	part2(parsed_input)

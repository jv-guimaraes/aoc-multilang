def parse_input(filename)
  groups = File.open(filename).read.split("\n\n")
  return groups.map { |g| g.split("\n")  }
end

def count_yes(group, type)
  yes_answers = Hash.new(0)
  for line in group
      line.split('').each{|answer| yes_answers[answer] += 1}
  end
  if type == 1 then return yes_answers.length end
  if type == 2 then return yes_answers.values.count{|v| v == group.length} end
end

groups = parse_input("input.txt")
puts "Part 1: #{groups.map{|g| count_yes(g, 1)}.sum}"
puts "Part 2: #{groups.map{|g| count_yes(g, 2)}.sum}"

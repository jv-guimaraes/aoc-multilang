def calc_seat_id(input, num_rows = 127, num_col= 7)
  seat_id = 8
  lo, hi = 0, num_rows
  for i in 0..6 do
    mid = lo + ((hi - lo) / 2)
    if input[i] == 'F' then hi = mid else lo = mid + 1 end
    # puts "#{input[i]}: [#{lo}, #{hi}]"
  end
  seat_id *= lo

  lo, hi = 0, num_col
  for i in 7..9 do
    mid = lo + ((hi - lo) / 2)
    if input[i] == 'L' then hi = mid else lo = mid + 1 end
    # puts "#{input[i]}: [#{lo}, #{hi}]"
  end
  seat_id += lo
  return seat_id
end

lines = File.open("input.txt").read.split("\n")
ids = lines.map { |line| calc_seat_id(line) }
puts "Part 1: #{ids.max}"

for i in 48..874 do
  if !ids.include?(i) then puts "Part 2: #{i}" end
end

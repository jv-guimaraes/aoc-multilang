def read_and_parse_input(filename)
  return File.open(filename).read.split("\n\n")
end

def valid_size?(passport)
  fields = passport.split(" ")
  if fields.length < 8
    return fields.length == 7 && fields.all? { |e| not e.start_with?("cid")}
  end
  return true
end

def valid_height?(height)
  h = height.to_i
  if !h.between?(59, 193) then return false end
  if height.end_with?("cm") && !h.between?(150, 193) then return false end
  if height.end_with?("in") && !h.between?(59, 76) then return false end
  return true
end

def valid_passport?(passport)
  if !valid_size?(passport) then return false end
  fields = passport.split(" ").map{ |p| p.split(":") }.to_h

  if !fields["byr"].to_i.between?(1920, 2002) then return false end

  if !fields["iyr"].to_i.between?(2010, 2020) then return false end

  if !fields["eyr"].to_i.between?(2020, 2030) then return false end

  if !valid_height?(fields["hgt"]) then return false end

  if !fields["hcl"].match?(/^#[0-9a-f]{6}$/) then return false end

  if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].include?(fields["ecl"])
    return false
  end

  if !fields["pid"].match?(/^[0-9]{9}$/) then return false end

  return true
end

passports = read_and_parse_input("input.txt")

puts "Part 1: #{passports.count { |p| valid_size?(p) }}"

puts "Part 2: #{passports.count { |p| valid_passport?(p) }}"

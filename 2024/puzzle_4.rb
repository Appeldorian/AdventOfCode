file_name = "./data/puzzle_4.txt"
# file_name = "./data/puzzle_4_test.txt"
data_string = File.readlines(file_name)

# xmas_map = Array.new(data_string.length) {Array.new(data_string[0].length)}
counter = 0

# for k in (0..3) 
#   puts k 
# end

# HORIZONTAL
puts "Horizontal"
for r in (0...data_string.size)
 for c in (0...data_string[r].length - 4)
  checked_string = data_string[r][c..c+3]
  if checked_string == "XMAS" || checked_string == "SAMX"
    puts checked_string
    counter += 1
  end
 end
end
puts "Counter: #{counter}"

# VERTICAL
puts "Vertical"
for r in (0...data_string.size - 3)
 for c in (0...data_string[r].length-1)
  checked_string = ""
  for k in (0..3)
    # puts "#{r+k} < #{data_string.size} - 3 = #{data_string.size-3} - #{c} < #{data_string[r].length}"
    # puts "#{r+k} #{c} - #{data_string[r+k][c]}"
    checked_string += data_string[r+k][c]
    # puts checked_string
  end
  if checked_string == "XMAS" || checked_string == "SAMX"
    # puts checked_string
    counter += 1
  end
 end
end
puts "Counter: #{counter}"


# DIAGONAL
puts "Diagonal"
for r in (0...data_string.size-3)
 for c in (0...data_string[r].length-4)
  checked_string = ""
  for k in (0..3)
    checked_string += data_string[r+k][c+k]
  end
  if checked_string == "XMAS" || checked_string == "SAMX"
    # puts checked_string
    counter += 1
  end

  checked_string = ""
  for k in (0..3)
    checked_string += data_string[r+k][c+3-k]
  end
  # puts "#{r}-#{r+3}, #{c+3}-#{c}  #{checked_string}"
  if checked_string == "XMAS" || checked_string == "SAMX"
    # puts checked_string
    counter += 1
  end
end
end
# puts "#{data_string.size} #{data_string[r].size}"

# DIAGONAL
def is_mas(checked_string) 
  if checked_string == "MAS" || checked_string == "SAM" 
    return true
  end
end
puts "X-MAS"
counter2 = 0
for r in (0...data_string.size-2)
 for c in (0...data_string[r].size-3)
  checked_string = ""
  for k in (0..2)
    checked_string += data_string[r+k][c+k]
  end
  checked_string2 = ""
  for k in (0..2)
    checked_string2 += data_string[r+k][c+2-k]
  end

  if is_mas(checked_string) && is_mas(checked_string2)
    counter2 += 1
  end
end
end
# puts "#{data_string.size} #{data_string[r].size}"
puts "Part 1: #{counter}"
puts "Part 2: #{counter2}"


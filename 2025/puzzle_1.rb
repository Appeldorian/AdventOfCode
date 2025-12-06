
file_name = "./data/puzzle_1.txt"
# file_name = "./data/puzzle_1_test.txt"
content = File.readlines(file_name)

dial = 50
puts dial


counter = 0
for line in content
  dial_start = dial
  sign = line[0]
  number = line[1..].to_i
  if sign == "L"
    dial = dial-number
  else
    dial = dial+number
  end

  counts = 0
  if dial <= 0
    counts += (dial - dial % 100).abs / 100
    if dial_start == 0 
      counts -= 1
    end
    if dial % 100 == 0
      counts += 1
    end
  elsif dial >= 100
    counts += (dial - dial % 100).abs / 100
    if dial == 0
      counts += 1
    end
  end

  counter += counts
  if counts >= 0
    puts "#{line[..-2]}: #{dial_start} -> #{dial} = #{dial%100} (added #{counts} counts) = #{counter}"
  end
  dial = dial % 100
end

puts "Puzzle score: #{counter}"
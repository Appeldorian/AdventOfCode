
file_name = "./data/puzzle_3.txt"
# content = File.readlines 
# content.each_with_index{|line, i| puts "#{i+1}: #{line}"}

def solve(multiples, part_nr)
  if part_nr == 1
    sum = 0
    skip_next = false
    for m in multiples
      if m == "do()"
        next
      end
      if m == "don't()"
        next
      end
      numbers = m[4..-2].split(",")
      sum += numbers[0].to_i * numbers[1].to_i
      # puts "#{numbers[0].to_i} * #{numbers[1].to_i} = #{numbers[0].to_i * numbers[1].to_i}"
    end

    puts "Puzzle 3 part 1: Sum: #{sum}"
  elsif part_nr == 2
    sum = 0
    skip_next = false
    for m in multiples
      if m == "do()"
        skip_next = false
        next
      end
      if m == "don't()"
        skip_next = true
        next
      end
      if !skip_next
        numbers = m[4..-2].split(",")
        sum += numbers[0].to_i * numbers[1].to_i
        # puts "#{numbers[0].to_i} * #{numbers[1].to_i} = #{numbers[0].to_i * numbers[1].to_i}"
      end
    end

    puts "Puzzle 3 part 2: Sum: #{sum}"
  else
    puts "Only part 1 or 2 are valid."
  end
end

data_string = File.read(file_name)
multiples = data_string.scan(Regexp.new("mul\\(\\d{1,9}\\,\\d{1,9}\\)|do\\(\\)|don't\\(\\)"))
if multiples.length == 0
  puts "No multiples found... Wrong regex probably"
else
  solve(multiples, 1)
  solve(multiples, 2)
end
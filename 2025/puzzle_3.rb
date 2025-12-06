
file_name = "./data/puzzle_3.txt"
# file_name = "./data/puzzle_3_test.txt"
content = File.readlines(file_name)

## Part 1
counter = 0

test = [1,2,3,4,5]
puts "#{test[-1]}"
puts "#{test[0..]}"

nr_batteries = 12

for line in content
  arr = line.strip.each_char.map(&:to_i)
  arr = arr[0..-1]

  nr_values = arr.length
  puts "#{line} ---- #{nr_values}"
  battery_values = []
  i = 0
  final_index = nr_values-1-nr_batteries

  nr_batteries_skipped = 0
  for k in 1..nr_batteries
    nr_batteries_to_choose = nr_batteries - k
    nr_batteries_available = nr_values - i
    final_index = (nr_batteries_available - nr_batteries_to_choose)
    search_area = arr[i..i+final_index-1]
    m = search_area.max
    puts "Battery #{k}/#{nr_batteries} Searching #{search_area.join} at i = #{i} and final_index = #{final_index} -> #{m}"
    mi = search_area.index(m)
    i += mi + 1
    nr_batteries_skipped += search_area.length - 1
    battery_values.append(m)
  end

  full = battery_values.map{|s| s.to_s}.join.to_i
  puts "-> #{full}"
  puts ""
  # puts "#{m}, #{i} and #{m2}, #{i2}"
  counter += full
end

puts "Puzzle score part 1: #{counter}"

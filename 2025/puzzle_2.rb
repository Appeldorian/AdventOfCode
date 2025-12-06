
file_name = "./data/puzzle_2.txt"
# file_name = "./data/puzzle_2_test.txt"
content = File.read(file_name)

## Part 1
ids = content.split(",")
counter = 0
for id in ids 
  split_id = id.split("-")
  rs = split_id[0]
  re = split_id[1]

  nr = rs
  while nr.to_i <= re.to_i
    if nr.length % 2 == 1
      nr = "1" + "0"*nr.length
      next
    end

    half = nr.length / 2
    first = nr[0..half-1]
    full = (first + first).to_i
    if rs.to_i <= full && full <= re.to_i
      # puts("#{id} - Duplicate: #{full}")
      counter += full
    end

    first = (first.to_i + 1).to_s
    nr = first + first  
  end
end

puts "Puzzle score part 1: #{counter}"


def get_divisors(number)
  arr = []
  for k in 2..number
    if number % k == 0
      arr.append(k)
    end
  end
  return arr
end

## Part 2
longest_number = 0
for id in ids
  split_id = id.split("-")
  rs = split_id[0]
  re = split_id[1]

  if re.length > longest_number
    longest_number = re.length
  end
end
puts "Longest number: #{longest_number}"
factors = []
for k in 2..longest_number
  factors += get_divisors(k)
end
factors = factors.uniq.sort
puts "Unique factors: #{factors}"

ids = content.split(",")
counter = 0
for id in ids 
  split_id = id.split("-")
  rs = split_id[0]
  re = split_id[1]

  invalid_ids = []
  # puts "#{id}: #{factors}"
  for factor in factors
    nr = rs
    while nr.to_i <= re.to_i
      if nr.length % factor != 0
        nr = "1" + "0"*nr.length
        next
      end

      part_index = nr.length / factor
      part = nr[0..part_index-1]

      if nr == part
        break
      end

      full = ""
      for k in 0..factor-1
        full += part
      end

      if rs.to_i <= full.to_i && full.to_i <= re.to_i && !invalid_ids.include?(full.to_i)
        puts("#{id} - factor #{factor} - invalid ID: #{full}")
        invalid_ids.append(full.to_i)
      end

      part = (part.to_i + 1).to_s
      full = ""
      for k in 0..factor-1
        full += part
      end
      # puts "New number:#{nr} -> #{full} (Part: #{part} x #{factor})"
      nr = full
      # sleep(0.5)
    end
  end
  counter += invalid_ids.uniq.sum
end


# ids = content.split(",")
# counter = 0
# for id in ids 
#   split_id = id.split("-")
#   rs = split_id[0]
#   re = split_id[1]

#   nr = rs
#   while nr.to_i <= re.to_i
#     if nr.length % 2 == 1
#       nr = "1" + "0"*nr.length
#       next
#     end

#     factors = get_divisors(nr.length)
#     # puts "#{nr}: #{factors}"

    # for factor in factors
    #   part = nr.length / factor
    #   full = ""
    #   for k in 0..factor-1
    #     full += nr[0..part-1]
    #   end

    #   if rs.to_i <= full.to_i && full.to_i <= re.to_i
    #     puts("#{id} - factor #{factor} - invalid ID: #{full}")
    #     counter += full.to_i
    #   end
    # end

#     first = nr[0..nr.length/2-1]
#     first = (first.to_i + 1).to_s
#     nr = first + first  
#     puts "New number: #{nr}"
#   end
# end

puts "Puzzle score part 2: #{counter}"

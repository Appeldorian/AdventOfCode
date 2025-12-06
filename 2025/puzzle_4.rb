# require 'matrix'
file_name = "./data/puzzle_4.txt"
# file_name = "./data/puzzle_4_test.txt"
content = File.readlines(file_name)

## Part 1
counter = 0

nr_rows = content.length
nr_cols = content[0].strip.length

m = Array.new(nr_rows) { Array.new(nr_cols) {|x| x = 0}}
sol = Array.new(nr_rows) { Array.new(nr_cols) {|x| x = 0}}

r = 0
for line in content
  arr = line.strip
  puts "#{arr}"
  for c in 0..nr_cols-1
    if arr[c] == "@"
      m[r][c] = 1
    else
      m[r][c] = 0
    end
  end
  r+=1
end

# sol = Marshal.load(Marshal.dump(m))

for r in 0..nr_rows-1
  r_min = [r-1, 0].max
  r_max = [r+1, nr_rows-1].min
  for c in 0..nr_cols-1
    c_min = [c-1, 0].max
    c_max = [c+1, nr_cols-1].min

    if m[r][c] == 0
      next
    end


    value = 0
    # puts "----"
    for k in r_min..r_max
      value += m[k][c_min..c_max].sum
      # puts "#{m[k][c_min..c_max]}"
    end

    value -= 1 # Don't count self
    # puts "value: #{value} -> #{value<4}"

    if value < 4
      sol[r][c] = 1
      counter += 1
    else
      sol[r][c] = 0
    end
  end
end

# for r in 0..nr_rows
#   puts "#{m[r]}"
# end

# for r in 0..nr_rows
#   puts "#{sol[r]}"
# end


puts "Puzzle score part 1: #{counter}"

total_papers = 0
for r in 0..nr_rows-1
  total_papers += m[r].sum
end
papers_removed = 0
while true
  papers_removed_this_iteration = 0
  for r in 0..nr_rows-1
    r_min = [r-1, 0].max
    r_max = [r+1, nr_rows-1].min
    for c in 0..nr_cols-1
      c_min = [c-1, 0].max
      c_max = [c+1, nr_cols-1].min

      if m[r][c] == 0
        next
      end


      value = 0
      for k in r_min..r_max
        value += m[k][c_min..c_max].sum
      end

      value -= 1 # Don't count self
      # puts "value: #{value} -> #{value<4}"

      if value < 4
        m[r][c] = 0
        papers_removed_this_iteration += 1
      end
    end
  end
  papers_removed += papers_removed_this_iteration
  if papers_removed_this_iteration == 0
    break
  end
end

for r in 0..nr_rows-1
  puts "#{m[r].join.to_s}"
end

puts "Part 2: Papers removed: #{papers_removed}"
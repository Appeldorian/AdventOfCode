# require 'matrix'
# file_name = "./data/puzzle_5_test.txt"
file_name = "./data/puzzle_5.txt"
content = File.readlines(file_name)


class Range
  attr_accessor :min_value, :max_value

  def initialize(min_value, max_value)
    @min_value, @max_value = min_value, max_value
  end

  def to_s
    "#{min_value} - #{max_value}"
  end

  def inspect
    "#{min_value} - #{max_value}"
  end

  def <=>(other)
    min_value <=> other.min_value
  end

  def check_within(value)
    return @min_value <= value && value <= @max_value
  end

  def check_overlap(other)
    return (min_value <= other.min_value && other.min_value <= max_value) ||
           (min_value <= other.max_value && other.max_value <= max_value) ||
           (other.min_value <= min_value && min_value <= other.max_value) ||
           (other.min_value <= max_value && max_value <= other.max_value)
  end

end

class CombinedRanges
  attr_accessor :ranges

  def initialize()
    @ranges = []
  end

  def add_range(new_range)
    # Merges ranges if they overlap.
    is_overlapping = false
    @ranges.each do |r|
      if r.check_overlap(new_range)
        r.max_value = [r.max_value, new_range.max_value].max
        r.min_value = [r.min_value, new_range.min_value].min
        is_overlapping = true
        break
      end
    end

    if !is_overlapping 
      @ranges.append(new_range)     
    end
  end

  def merge_ranges
    all_ranges = Ranges.new()
    @ranges.each do |r|
      all_ranges.add_range(r)
    end
    if @ranges.length != all_ranges.ranges.length
      puts "Reduced from #{@ranges.length} to #{all_ranges.ranges.length}"
      @ranges = all_ranges.ranges.map(&:clone)
      self.merge_ranges
    end
  end

  def sort_ranges
    @ranges = @ranges.sort
  end

end

# Parse data
ranges = []
ingredients = []
r = 0
for line in content
  arr = line.strip
  if arr == ""
    next
  end
  if (arr.include? "-")
    arr_split = arr.split("-")
    r = Range.new(arr_split[0].to_i, arr_split[1].to_i)
    ranges.append(r)
  else
    ingredients.append(arr.to_i)
  end
end

puts "Found #{ranges.length} ranges and #{ingredients.length} ingredients."

# Part a
counter = 0
for i in ingredients
  is_fresh = false
  for r in ranges
    if r.check_within(i)
      is_fresh = true
      break
    end
  end
  if is_fresh
    counter += 1
  end
end

puts "Puzzle 5a: #{counter}"

# Part b
combined_ranges = CombinedRanges.new()
for r in ranges
  combined_ranges.add_range(r)
end
combined_ranges.merge_ranges

counter2 = 0
for r in combined_ranges.ranges 
  counter2 += r.max_value - r.min_value + 1
end

for r in ranges
  is_overlapping = false
  for c in combined_ranges.ranges
    if r.check_overlap(c) 
      is_overlapping = true
      break
    end
  end

  if !is_overlapping
    puts "No overlapping range found for #{r}"
  end
end

puts "Puzzle 5b: #{counter2}"
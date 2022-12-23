require 'csv'
require_relative 'modifier'

# TODO
# * parse row-level operations
# * insert variables
# * handle supplied variables
module GSPush
  class Cell
    attr_reader :modifier
    attr_reader :value

    def self.parse_cell(v)
      if modifier = Modifier.get_modifier_from_value(v)
        Cell.new(modifier.value_without_modifier, modifier)
      else 
        Cell.new(v)
      end
    end

    def initialize(value, modifier = nil)
      @value = value
      @modifier = modifier
    end

    def to_s
      "#{@value} #{@modifier}"
    end
  end

  class Row 
    attr_accessor :cells
    attr_accessor :modifier

    def self.parse_row(row)
      cells = row.map {|cell| Cell.parse_cell(cell)}

      # XXX handle the row-level modifier
      Row.new(cells)
    end

    def initialize(cells, modifier = nil)
      @cells = cells
      @modifier = modifier
    end

    def to_s
      @cells
    end
  end

  class Template
    attr_accessor :rows

    def initialize(input, key_values: {})
      @input = input
      @key_values = key_values
      @rows = []
    end

    def process!
      @rows = CSV.new(@input).map do |row|
        Row.parse_row(row)
      end
    end

    def get_all_values
      @rows
    end
  end
end

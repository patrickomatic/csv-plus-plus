# frozen_string_literal: true

require_relative 'cell'
require_relative 'modifier.tab'

module CSVPlusPlus
  ##
  # A row of a template
  class Row
    attr_reader :cells, :index, :modifier

    # initialize
    def initialize(index, cells, modifier)
      @cells = cells
      @modifier = modifier
      @index = index
    end

    # Set the row index. And update the index of all affected cells
    def index=(index)
      @index = index
      @cells.each { |cell| cell.row_index = index }
    end

    # How much this row will expand itself, if at all (0)
    def expand_amount
      return 0 unless @modifier.expand

      @modifier.expand.repetitions || (1000 - @index)
    end

    # to_s
    def to_s
      "Row(index: #{index}, modifier: #{modifier}, cells: #{cells})"
    end

    # Return a deep copy of this row
    def deep_clone
      ::Marshal.load(::Marshal.dump(self))
    end
  end
end

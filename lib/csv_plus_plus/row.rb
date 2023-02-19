# frozen_string_literal: true

require_relative 'cell'
require_relative 'modifier.tab'

module CSVPlusPlus
  # A row of a template
  #
  # @attr_reader cells [Array<Cell>]
  # @attr_reader index [Integer] The index of this row
  # @attr_reader modifier [Modifier] The modifier to apply to all cells in this row
  class Row
    attr_reader :cells, :index, :modifier

    # @param index [Integer] The index of this row (starts at 0)
    # @param cells [Array<Cell>] The cells belonging to this row
    # @param modifier [Modifier] The modifier to apply to all cells in this row
    def initialize(index, cells, modifier)
      @cells = cells
      @modifier = modifier
      @index = index
    end

    # Set the row's +index+ and update the +row_index+ of all affected cells
    # @param index [Integer] The index of this row (starts at 0)
    def index=(index)
      @index = index
      @cells.each { |cell| cell.row_index = index }
    end

    # How much this row will expand itself, if at all (0)
    # @return [Integer]
    def expand_amount
      return 0 unless @modifier.expand

      @modifier.expand.repetitions || (1000 - @index)
    end

    # @return [String]
    def to_s
      "Row(index: #{index}, modifier: #{modifier}, cells: #{cells})"
    end

    # Return a deep copy of this row
    # @return [Row]
    def deep_clone
      ::Marshal.load(::Marshal.dump(self))
    end
  end
end

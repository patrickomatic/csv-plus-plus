# frozen_string_literal: true

require_relative 'cell'
require_relative 'modifier.tab'

module CSVPlusPlus
  ##
  # A row of a template
  class Row
    attr_reader :cells, :index, :modifier

    # Using the current +compiler+ and the given +csv_row+ parse it into a Row of Cells
    # +csv_row+ should have already been run through a CSV parser and is an array of strings
    def self.parse(csv_row, compiler)
      row_modifier = ::CSVPlusPlus::Modifier.new(row_level: true)

      cells =
        compiler.map_row(csv_row) do |value, cell_index|
          cell_modifier = ::CSVPlusPlus::Modifier.new
          parsed_value = ::CSVPlusPlus::ModifierParser.new.parse(value, compiler:, row_modifier:, cell_modifier:)

          ::CSVPlusPlus::Cell.new(compiler.row_index, cell_index, parsed_value, cell_modifier)
        end

      new(compiler.row_index, cells, row_modifier)
    end

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

require_relative 'modifier.tab'
require_relative 'cell'

module CSVPlusPlus
  class Row 
    attr_reader :cells, :index, :modifier

    def initialize(index, cells, modifier)
      @cells = cells
      @modifier = modifier
      @index = index
    end

    def self.parse(csv_row, execution_context)
      row_modifier = Modifier.new(row_level: true)

      cells = execution_context.map_row(csv_row) do |value, cell_index|
        cell_modifier = Modifier.new
        parsed_value = ModifierParser.new.parse(value, execution_context:,
                                                row_modifier:, cell_modifier:)
        Cell.new(execution_context.row_index, 
                 cell_index, parsed_value, cell_modifier)
      end

      Row.new(execution_context.row_index, cells, row_modifier)
    end

    def index=(i)
      @index = i
      @cells.each {|cell| cell.row_index = i}
    end

    def expand_amount
      return 0 unless @modifier.expand
      @modifier.expand.repetitions || 1000 - @index
    end

    def to_s
      "Row(index: #{index.to_s}, modifier: #{modifier.to_s}, cells: #{cells.to_s})"
    end

    def deep_clone
      Marshal.load(Marshal.dump(self))
    end
  end
end 

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

    def self.parse_row(csv_row, row_number)
      row_modifier = Modifier.new(row_level: true)

      cells = csv_row.map.with_index do |value, cell_number|
        cell_modifier = Modifier.new
        parsed_value = ModifierParser.new.parse(value, 
                                                row_modifier:, cell_modifier:,
                                                row_number:, cell_number:)
        Cell.new(row_number, cell_number, parsed_value, cell_modifier)
      end

      Row.new(row_number, cells, row_modifier)
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
      @cells
    end

    def deep_clone
      Marshal.load(Marshal.dump(self))
    end
  end
end 

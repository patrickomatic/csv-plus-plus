require_relative 'modifier.tab'
require_relative 'cell'

module CSVPlusPlus
  class Row 
    attr_reader :cells, :modifier

    def self.parse_row(csv_row, row_number)
      row_modifier = nil

      cells = csv_row.map.with_index do |value, cell_number|
        modifier = ModifierParser.new.parse(value, row_number, cell_number)

        if modifier && modifier.row_level?
          row_modifier = modifier
          Cell.new value
        elsif modifier
          Cell.new(value, modifier)
        else
          Cell.new value
        end
      end

      Row.new(cells, row_modifier)
    end

    def initialize(cells, modifier = nil)
      @cells = cells
      @modifier = modifier
    end

    def to_s
      @cells
    end
  end
end 

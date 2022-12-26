require_relative 'modifier'
require_relative 'cell'

module GSPush
  class Row 
    attr_reader :cells
    attr_reader :modifier

    def self.parse_row(csv_row)
      row_modifier = nil
      cells = csv_row.map do |value|
        modifier = Modifier.get_modifier_from_value(value)
        if modifier && modifier.row_level?
          row_modifier = modifier
          Cell.new(value)
        elsif modifier
          Cell.new(value, modifier)
        else
          Cell.new(value)
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

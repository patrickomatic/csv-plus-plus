require_relative 'modifier.tab'
require_relative 'cell'

module CSVPlusPlus
  class Row 
    attr_reader :cells, :modifier

    def self.parse_row(csv_row, row_number)
      row_modifier = Modifier.new(row_level: false)

      cells = csv_row.map.with_index do |value, cell_number|
        cell_modifier = row_modifier.clone_defaults_from
        parsed_value = ModifierParser.new.parse(value, row_modifier:, cell_modifier:,
                                                row_number:, cell_number:)
        Cell.new(parsed_value, cell_modifier)
      end

      Row.new(cells, row_modifier)
    end

    def initialize(cells, modifier)
      @cells = cells
      @modifier = modifier
    end

    def to_s
      @cells
    end
  end
end 

require 'csv'

# TODO
# * parse row-level operations
# * insert variables
# * handle supplied variables
module GSPush
  class Cell
    CELL_MODIFIER_REGEX = /^\<\[(?:\/?(?<align>align)=(?<align_value>left|center|right))*
                                (?:\/?(?<formats>format)=(?:\s*(?<formats_value>bold|italic|underline))*)*
                              \]\>(?<cell_value>.*)/x

    attr_accessor :value
    attr_accessor :formats
    attr_accessor :align

    def self.parse_cell(v)
      if match = v.match(CELL_MODIFIER_REGEX)
        captures = match.named_captures
        align = captures["align"].nil? ? nil : captures["align_value"]
        formats = captures["formats"].nil? ? [] : [captures["formats_value"]]
        Cell.new(captures["cell_value"], align: align, formats: formats)
      else 
        Cell.new(v)
      end
    end

    # XXX inject in a Formattable that handles the aligns and format
    def initialize(value, formats: [], align: nil)
      @value = value
      @formats = formats
      @align = align
    end

    def to_s
      "#{value}#{formats.length > 0 ? " formats=#{formats}" : ""}#{align.length > 0 ? " align=#{align}" : ""}"
    end
  end

  class Row 
    # XXX need to handle them joined together with a slash
    ROW_MODIFIER_REGEX = /^\<\!\[ (align=(left|center|right))
                                | (format=(bold|italic|underline)(\s bold|italic|underscore)*)
                                | (range=(\d+(:\d+)?))
                               \]\>.*/x

    attr_accessor :cells
    attr_accessor :formats
    attr_accessor :align
    attr_accessor :range

    def self.parse_row(row)
      cells = row.map {|cell| Cell.parse_cell(cell)}
      formats = []
      aligns = []
      range = []

      if cells.length > 1 && match = cells[0].value.match(ROW_MODIFIER_REGEX)
        # XXX need to remove the matched modifier
        puts "modified the row", match
      end

      Row.new(cells, formats: formats, aligns: aligns, range: range)
    end

    def initialize(cells, formats: [], aligns: [], range: nil)
      @cells = cells
      @formats = formats
      @aligns = aligns
      @range = range
    end

    def to_s
      "#{@cells}"
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

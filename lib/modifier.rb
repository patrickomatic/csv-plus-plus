module GSPush
  Range = Struct.new(:start_row, :end_row) do
    def to_s
      "Row range (#{start} - #{end_row})"
    end
  end

  class ModifierError < StandardError
    attr_reader :cell_input

    def initialize(cell_input)
      @cell_input = cell_input
    end
  end

  class Modifier
    attr_reader :formats 
    attr_reader :align
    attr_reader :value_without_modifier
    attr_accessor :foreground_color
    attr_accessor :range

    MODIFIER_REGEX = /^\<(?<is_row_level>\!)\[(?<modifiers>\w+)\]\>(?<cell_value>)$/
#    CELL_MODIFIER_REGEX = /^\<\[(?:\/?(?<align>align)=(?<align_value>left|center|right))*
#                                (?:\/?(?<formats>format)=(?:\s*(?<formats_value>bold|italic|underline))*)*
#                              \]\>(?<cell_value>.*)/x


    def self.get_modifier_from_value(value)
      match = value.match(MODIFIER_REGEX)
      return nil unless match

      puts match 
    end

    def initialize(formats, align)
      parse_cell_value!(cell_value)
    end

    def parse_cell_value!(str)
    end

    def formats=(format_value)
      # XXX allow it to be an array or single, add it and de-dupe
      # XXX make sure it's a valid value
      formats.push(formats_value)
    end

    def align=(align)
      unless ['left', 'center', 'right'].include?(align)
        throw ModifierError.new("Invalid value: #{align}")
      end
      @align = align
    end

    def bold?
      @formats.include? 'bold'
    end

    def italic?
      @formats.include? 'italic'
    end

    def strikethrough?
      @formats.include? 'strikethrough'
    end

    def underline?
      @formats.include? 'underline'
    end
  end
end

module GSPush
  Range = Struct.new(:start_row, :end_row) do
    def to_s
      "Row range (#{start} - #{end_row})"
    end
  end

  class Modifier
    class SyntaxError < StandardError
      attr_reader :cell_input

      def initialize(cell_input)
        @cell_input = cell_input
      end
    end

    attr_reader :formats 
    attr_reader :align
    attr_reader :value_without_modifier
    attr_accessor :foreground_color
    attr_accessor :range

    def self.get_modifier_from_value(value)
      match = value.match(MODIFIER_REGEX)
      return nil unless match
      re_groups = match.named_captures

      formats, align, range = [], nil, range
      modifiers = re_groups["modifiers"].split("/").map {|kv| kv.split("=")}.map do |k, v|
        case k
        when "format"
          fs = v.split(/\s+/)
          fs.each do |f|
            unless ['bold', 'underline', 'italic', 'strikethrough'].include?(f)
              raise SyntaxError.new "Invalid format modifier: #{f}"
            end
          end
          formats += fs
        when "align"
          unless ['left', 'center', 'right'].include?(v)
            raise SyntaxError.new "Invalid align modifier: #{align}"
          end
          align = v
        when "range"
          # XXX handle a range
          range = nil
        else
          raise SyntaxError.new "Unknown modifier: #{v}"
        end
      end
      
      Modifier.new(re_groups["cell_value"], 
                   formats: formats, 
                   align: align,
                   range: range)
    end

    def initialize(value_without_modifier, formats: [], align: nil, range: nil)
      @value_without_modifier = value_without_modifier
      @formats = formats
      @align = align
      @range = range
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

    private

    MODIFIER_REGEX = /^\<\[(?<modifiers>.+)\]\>(?<cell_value>.*)$/
  end
end

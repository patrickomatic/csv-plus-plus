module GSPush
  ExpandRange = Struct.new(:start_row, :end_row) do
    def to_s
      "Expand range (#{start} - #{end_row})"
    end
  end

  class Modifier
    MODIFIER_REGEX = /^(?<row_level>\!)?\[\[(?<modifiers>.+)\]\](?<cell_value>.*)$/

    class SyntaxError < StandardError
      attr_reader :cell_input

      def initialize(cell_input)
        @cell_input = cell_input
      end
    end

    attr_reader :formats 
    attr_reader :align
    attr_reader :value_without_modifier
    attr_reader :foreground_color
    attr_reader :expand

    def self.get_modifier_from_value(value)
      match = value.match(MODIFIER_REGEX)
      return nil unless match
      re_groups = match.named_captures

      row_level = !re_groups["row_level"].nil?
      formats, align, expand = [], nil, expand
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
            raise SyntaxError.new "Invalid align modifier: #{v}"
          end
          align = v
        when "expand"
          unless match = v.match(/^(\d+):(\d+)?/)
            raise SyntaxError.new "Invalid expand modifier: #{v}"
          end
          start, end_row = match[1], match[2]
          expand = ExpandRange.new(start.to_i, 
                                   end_row.nil? ? nil : end_row.to_i)
        else
          raise SyntaxError.new "Unknown modifier: #{v}"
        end
      end
      
      Modifier.new(re_groups["cell_value"], 
                   formats: formats, 
                   align: align,
                   expand: expand,
                   row_level: row_level)
    end

    def initialize(value_without_modifier, formats: [], align: nil, expand: nil, row_level: false)
      @value_without_modifier = value_without_modifier
      @formats = formats
      @align = align
      @expand = expand
      @row_level = row_level
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

    def row_level?
      @row_level
    end

    def cell_level?
      !@row_level
    end
  end
end

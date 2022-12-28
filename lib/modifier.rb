require_relative 'syntax_error'

module GSPush
  Expand = Struct.new(:repetitions) do
    def infinite?
      repetitions.nil?
    end

    def to_s
      "Expand #{repetitions || 'infinity'}"
    end
  end

  class Modifier
    MODIFIER_REGEX = /^(?<row_level>\!)?\[\[(?<modifiers>.+)\]\](?<cell_value>.*)$/
    attr_reader :formats, :align, :value_without_modifier, :foreground_color, :expand

    def self.get_modifier_from_value(value, row_number, cell_number)
      return nil unless value

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
              raise SyntaxError.new( "Invalid `format` modifier", f, row_number:, cell_number:)
            end
          end
          formats += fs
        when "align"
          unless ['left', 'center', 'right'].include?(v)
            raise SyntaxError.new("Invalid `align` modifier", v, row_number:, cell_number:)
          end
          align = v
        when "expand"
          if !v.nil? and !(m = v.match(/^(\d+)?/))
            raise SyntaxError.new("Invalid `expand` modifier", v, row_number:, cell_number:)
          end
          expand = Expand.new(m.nil? ? nil : m[1].to_i)
        else
          raise SyntaxError.new('Unknown modifier', value, row_number:, cell_number:)
        end
      end
      
      Modifier.new(re_groups["cell_value"], formats:, align:, expand:, row_level:)
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

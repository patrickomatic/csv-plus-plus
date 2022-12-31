require 'set'
require_relative 'syntax_error'

module CSVPlusPlus
  class Modifier
    Expand = Struct.new(:repetitions) do
      def infinite?
        repetitions.ni?
      end

      def to_s
        "Expand #{repetitions || 'infinity'}"
      end
    end

    attr_accessor :bordersize,
                  :borderstyle,
                  :expand,
                  :font,
                  :fontcolor,
                  :fontfamily,
                  :hyperlink,
                  :note,
                  :validation

    def initialize
      @row_level = false
      @freeze = false
      @align = Set.new
      @borders = Set.new
      @formats = Set.new
    end

    def align
      @align.to_a
    end

    def align=(value)
      @align << value
    end

    def borders
      @borders.to_a
    end

    def borders=(value)
      @borders << value
    end

    def formats
      @formats.to_a
    end

    def formats=(value)
      @formats << value
    end

    def freeze!
      @frozen = true
    end

    def frozen?
      @frozen
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

    def row_level!
      @row_level = true
    end

    def row_level?
      @row_level
    end

    def cell_level?
      !@row_level
    end
  end
end

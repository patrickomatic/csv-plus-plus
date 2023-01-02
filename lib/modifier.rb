require 'set'
require_relative 'syntax_error'

module CSVPlusPlus
  class Modifier
    Expand = Struct.new(:repetitions) do
      def infinite?
        repetitions.nil?
      end

      def to_s
        "Expand #{repetitions || 'infinity'}"
      end
    end

    attr_accessor :bordersize,
                  :borderstyle,
                  :expand,
                  :fontcolor,
                  :fontfamily,
                  :fontsize,
                  :hyperlink,
                  :note,
                  :row_level,
                  :validation

    def initialize(row_level: false)
      @row_level = row_level
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
      @borders
    end

    def border=(value)
      @borders << value
    end

    def border_all?
      @borders.include? 'all'
    end

    def border_top?
      border_all? || @borders.include?('top')
    end

    def border_right?
      border_all? || @borders.include?('right')
    end

    def border_bottom?
      border_all? || @borders.include?('bottom')
    end

    def border_left?
      border_all? || @borders.include?('left')
    end

    def has_border?
      !@borders.empty?
    end

    def formats
      @formats
    end

    def format=(value)
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

    def borderstyle
      @borderstyle || 'solid'
    end

    def take_defaults_from!(m)
      %i[
        @align 
        @borderstyle 
        @borders
        @formats
        @bordersize
        @borderstyle
        @fontcolor
        @fontfamily
        @fontsize
        @hyperlink
        @note
        @validation
      ].each do |property|
        value = m.instance_variable_get property
        self.instance_variable_set(property, value.clone)
      end
    end
  end
end

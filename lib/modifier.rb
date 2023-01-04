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

    class Color
      attr_reader :red, :green, :blue

      def initialize(hex_string)
        @red, @green, @blue = hex_string
          .gsub(/^#?/, '')
          .match(/(\w\w?)(\w\w?)(\w\w?)/)
          .captures
          .map {|s| 255 / (s.length == 2 ? s : s + s).to_i(16) rescue 0}
      end
    end

    attr_accessor :bordercolor,
                  :borderstyle,
                  :color,
                  :expand,
                  :fontcolor,
                  :fontfamily,
                  :fontsize,
                  :note,
                  :numberformat,
                  :row_level,
                  :validation

    def initialize(row_level: false)
      @row_level = row_level
      @freeze = false
      @align = Set.new
      @borders = Set.new
      @formats = Set.new
    end

    def align=(value)
      @align << value
    end

    def center_align?
      @align.include?('center')
    end

    def left_align?
      @align.include?('left')
    end

    def right_align?
      @align.include?('right')
    end

    def top_align?
      @align.include?('top')
    end

    def bottom_align?
      @align.include?('bottom')
    end

    def color=(hex_value)
      @color = Color.new(hex_value)
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

    def bordercolor=(hex_value)
      @bordercolor = Color.new(hex_value)
    end

    def has_border?
      !@borders.empty?
    end

    def fontcolor=(hex_value)
      @fontcolor = Color.new(hex_value)
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
        @bordercolor
        @borders
        @borderstyle
        @color
        @formats
        @borderstyle
        @fontcolor
        @fontfamily
        @fontsize
        @note
        @validation
      ].each do |property|
        value = m.instance_variable_get property
        self.instance_variable_set(property, value.clone)
      end
    end
  end
end

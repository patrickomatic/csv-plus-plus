# frozen_string_literal: true

module CSVPlusPlus
  # A color value
  class Color
    attr_reader :red, :green, :blue

    # create an instance from a string like "#FFF" or "#FFFFFF"
    def initialize(hex_string)
      @red, @green, @blue =
        hex_string
        .gsub(/^#?/, '')
        .match(/(\w\w?)(\w\w?)(\w\w?)/)
        .captures
        .map { |s| (s.length == 2 ? s : s + s).to_i(16) / 255.0 }
    end

    # to_s
    def to_s
      "Color(r: #{@red}, g: #{@green}, b: #{@blue})"
    end

    # ==
    def ==(other)
      other.is_a?(self.class) && other.red == @red && other.green == @green && other.blue == @blue
    end
  end
end

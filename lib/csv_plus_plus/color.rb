# frozen_string_literal: true

module CSVPlusPlus
  # A color value parsed into it's respective components
  #
  # attr_reader blue_hex [String] The blue value in hex ("FF", "00", "AF", etc)
  # attr_reader green_hex [String] The green value in hex ("FF", "00", "AF", etc)
  # attr_reader red_hex [String] The red value in hex ("FF", "00", "AF", etc)
  class Color
    attr_reader :red_hex, :green_hex, :blue_hex

    # create an instance from a string like "#FFF" or "#FFFFFF"
    #
    # @param hex_string [String] The hex string input to parse
    def initialize(hex_string)
      @red_hex, @green_hex, @blue_hex = hex_string
                                        .gsub(/^#?/, '')
                                        .match(/([0-9a-f]{1,2})([0-9a-f]{1,2})([0-9a-f]{1,2})/i)
                                        &.captures
                                        &.map { |s| s.length == 1 ? s + s : s }
    end

    # The percent (decimal between 0-1) of red
    #
    # @return [Numeric]
    def red_percent
      hex_to_percent(@red_hex)
    end

    # The percent (decimal between 0-1) of green
    #
    # @return [Numeric]
    def green_percent
      hex_to_percent(@green_hex)
    end

    # The percent (decimal between 0-1) of blue
    #
    # @return [Numeric]
    def blue_percent
      hex_to_percent(@blue_hex)
    end

    # Create a hex representation of the color (without a '#')
    #
    # @return [String]
    def to_hex
      [@red_hex, @green_hex, @blue_hex].join
    end

    # @return [String]
    def to_s
      "Color(r: #{@red_hex}, g: #{@green_hex}, b: #{@blue_hex})"
    end

    # @return [boolean]
    def ==(other)
      other.is_a?(self.class) &&
        other.red_hex == @red_hex &&
        other.green_hex == @green_hex &&
        other.blue_hex == @blue_hex
    end

    private

    def hex_to_percent(hex)
      hex.to_i(16) / 255.0
    end
  end
end

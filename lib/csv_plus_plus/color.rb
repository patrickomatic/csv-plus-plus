# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # A color value parsed into it's respective components
  #
  # attr_reader blue_hex [String] The blue value in hex ("FF", "00", "AF", etc)
  # attr_reader green_hex [String] The green value in hex ("FF", "00", "AF", etc)
  # attr_reader red_hex [String] The red value in hex ("FF", "00", "AF", etc)
  class Color
    extend ::T::Sig

    sig { returns(::String) }
    attr_reader :red_hex

    sig { returns(::String) }
    attr_reader :green_hex

    sig { returns(::String) }
    attr_reader :blue_hex

    HEX_STRING_REGEXP = /^#?([0-9a-f]{1,2})([0-9a-f]{1,2})([0-9a-f]{1,2})/i
    public_constant :HEX_STRING_REGEXP

    sig { params(hex_string: ::String).returns(::T::Boolean) }
    # Is +hex_string+ a valid hexadecimal color code? This function will accept input like the 6-digit format: #FF00FF,
    # 00AABB and the shorter 3-digit format: #FFF, 0FA.
    #
    # @param hex_string [::String] The string to see if it's valid hex string
    #
    # @return [boolean]
    def self.valid_hex_string?(hex_string)
      !(hex_string.strip =~ ::CSVPlusPlus::Color::HEX_STRING_REGEXP).nil?
    end

    sig { params(hex_string: ::String).void }
    # Create an instance from a string like "#FFF" or "#FFFFFF"
    #
    # @param hex_string [String] The hex string input to parse
    # rubocop:disable Metrics/CyclomaticComplexity
    def initialize(hex_string)
      red_hex, green_hex, blue_hex = hex_string.strip.match(::CSVPlusPlus::Color::HEX_STRING_REGEXP)
                                        &.captures
                                        &.map { |s| s.length == 1 ? s + s : s }
      raise(::CSVPlusPlus::Error::Error, "Invalid color: #{hex_string}") unless red_hex && green_hex && blue_hex

      @red_hex = ::T.let(red_hex, ::String)
      @green_hex = ::T.let(green_hex, ::String)
      @blue_hex = ::T.let(blue_hex, ::String)
    end
    # rubocop:enable Metrics/CyclomaticComplexity

    sig { returns(::Float) }
    # The percent (decimal between 0-1) of red
    #
    # @return [Numeric]
    def red_percent
      hex_to_percent(@red_hex)
    end

    sig { returns(::Float) }
    # The percent (decimal between 0-1) of green
    #
    # @return [Numeric]
    def green_percent
      hex_to_percent(@green_hex)
    end

    sig { returns(::Float) }
    # The percent (decimal between 0-1) of blue
    #
    # @return [Numeric]
    def blue_percent
      hex_to_percent(@blue_hex)
    end

    sig { returns(::String) }
    # Create a hex representation of the color (without a '#')
    #
    # @return [::String]
    def to_hex
      [@red_hex, @green_hex, @blue_hex].join
    end

    sig { params(other: ::Object).returns(::T::Boolean) }
    # @return [boolean]
    def ==(other)
      other.is_a?(self.class) &&
        other.red_hex == @red_hex &&
        other.green_hex == @green_hex &&
        other.blue_hex == @blue_hex
    end

    private

    sig { params(hex: ::String).returns(::Float) }
    def hex_to_percent(hex)
      hex.to_i(16) / 255.0
    end
  end
end

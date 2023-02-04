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
        .map do |s|
          255 / (s.length == 2 ? s : s + s).to_i(16)
        rescue ::StandardError
          0
        end
    end
  end
end

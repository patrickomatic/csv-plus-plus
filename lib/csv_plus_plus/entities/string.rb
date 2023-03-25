# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A string value
    #
    # @attr_reader value [String]
    class String < Entity
      attr_reader :value

      # @param value [String] The string that has been parsed out of the template
      def initialize(value)
        super(:string)

        @value = value.gsub(/^"|"$/, '')
      end

      # @param _runtime [Runtime]
      #
      # @return [::String]
      def evaluate(_runtime)
        "\"#{@value}\""
      end

      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        super && value == other.value
      end
    end
  end
end

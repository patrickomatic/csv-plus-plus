# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A number value
    #
    # @attr_reader value [Numeric] The parsed number value
    class Number < Entity
      attr_reader :value

      # @param value [String, Numeric] Either a +String+ that looks like a number, or an already parsed Numeric
      def initialize(value)
        super(:number)

        @value =
          if value.instance_of?(::String)
            value.include?('.') ? Float(value) : Integer(value, 10)
          else
            value
          end
      end

      # @return [String]
      def to_s
        @value.to_s
      end

      # @return [boolean]
      def ==(other)
        super && value == other.value
      end
    end
  end
end

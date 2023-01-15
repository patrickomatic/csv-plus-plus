# frozen_string_literal: true

module CSVPlusPlus
  module Language
    ##
    # A number value
    class Number < Entity
      attr_reader :value

      # initialize
      def initialize(value)
        super(:number)
        @value =
          if value.instance_of?(::String)
            value.include?('.') ? Float(value) : Integer(value, 10)
          else
            value
          end
      end

      # to_s
      def to_s
        @value.to_s
      end

      # ==
      def ==(other)
        super && value == other.value
      end
    end
  end
end

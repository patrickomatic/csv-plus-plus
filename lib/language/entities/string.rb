# frozen_string_literal: true

module CSVPlusPlus
  module Language
    ##
    # A string value
    class String < Entity
      attr_reader :value

      # initialize
      def initialize(value)
        super(:string)
        @value = value.gsub(/^"|"$/, '')
      end

      # to_s
      def to_s
        "\"#{@value}\""
      end

      # ==
      def ==(other)
        super && value == other.value
      end
    end
  end
end

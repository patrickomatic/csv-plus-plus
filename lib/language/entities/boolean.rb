# frozen_string_literal: true

require_relative './entity'

module CSVPlusPlus
  module Language
    ##
    # A boolean value
    class Boolean < Entity
      attr_reader :value

      # initialize
      def initialize(value)
        super(:boolean)
        # TODO: probably can do a lot better in general on type validation
        @value = value.is_a?(::String) ? (value.downcase == 'true') : value
      end

      # to_s
      def to_s
        @value.to_s.upcase
      end

      # ==
      def ==(other)
        super && value == other.value
      end
    end
  end
end

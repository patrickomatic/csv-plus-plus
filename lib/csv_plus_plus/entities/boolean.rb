# typed: true
# frozen_string_literal: true

require_relative './entity'

module CSVPlusPlus
  module Entities
    # A boolean value
    #
    # @attr_reader value [true, false]
    class Boolean < Entity
      attr_reader :value

      # @param value [String, boolean]
      def initialize(value)
        super(:boolean)
        # TODO: probably can do a lot better in general on type validation
        @value = value.is_a?(::String) ? (value.downcase == 'true') : value
      end

      # @param _runtime [Runtime]
      #
      # @return [String]
      def evaluate(_runtime)
        @value.to_s.upcase
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

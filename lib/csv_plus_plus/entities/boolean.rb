# typed: strict
# frozen_string_literal: true

require_relative './entity'

module CSVPlusPlus
  module Entities
    # A boolean value
    #
    # @attr_reader value [true, false]
    class Boolean < Entity
      sig { returns(::T::Boolean) }
      attr_reader :value

      sig { params(value: ::T.any(::String, ::T::Boolean)).void }
      # @param value [String, boolean]
      def initialize(value)
        super(:boolean)
        # TODO: probably can do a lot better in general on type validation
        @value = ::T.let(value.is_a?(::String) ? (value.downcase == 'true') : value, ::T::Boolean)
      end

      sig { override.params(_runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      # @param _runtime [Runtime]
      #
      # @return [String]
      def evaluate(_runtime)
        @value.to_s.upcase
      end

      sig { params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        return false unless super

        other.is_a?(self.class) && value == other.value
      end
    end
  end
end

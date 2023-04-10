# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A number value
    #
    # @attr_reader value [Numeric] The parsed number value
    class Number < Entity
      sig { returns(::Numeric) }
      attr_reader :value

      sig { params(value: ::T.any(::String, ::Numeric)).void }
      # @param value [String, Numeric] Either a +String+ that looks like a number, or an already parsed Numeric
      def initialize(value)
        super(::CSVPlusPlus::Entities::Type::Number)

        @value =
          ::T.let(
            (if value.is_a?(::String)
               value.include?('.') ? Float(value) : Integer(value, 10)
             else
               value
             end),
            ::Numeric
          )
      end

      sig { override.params(_runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      # @param _runtime [Runtime]
      #
      # @return [::String]
      def evaluate(_runtime)
        @value.to_s
      end

      sig { override.params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [::T::Boolean]
      def ==(other)
        return false unless super

        other.is_a?(self.class) && @value == other.value
      end
    end
  end
end

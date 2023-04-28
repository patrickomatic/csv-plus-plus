# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A number value
    #
    # @attr_reader value [Numeric] The parsed number value
    class Number < ::CSVPlusPlus::Entities::Entity
      extend ::T::Sig

      sig { returns(::Numeric) }
      attr_reader :value

      sig { params(value: ::T.any(::String, ::Numeric)).void }
      # @param value [String, Numeric] Either a +String+ that looks like a number, or an already parsed Numeric
      def initialize(value)
        super()

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

      sig { override.params(_position: ::CSVPlusPlus::Runtime::Position).returns(::String) }
      # @param _position [Position]
      #
      # @return [::String]
      def evaluate(_position)
        @value.to_s
      end

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [BasicObject]
      #
      # @return [::T::Boolean]
      def ==(other)
        case other
        when self.class
          @value == other.value
        else
          false
        end
      end
    end
  end
end

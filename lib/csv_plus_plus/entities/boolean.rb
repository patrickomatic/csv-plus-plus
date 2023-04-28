# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A boolean value
    #
    # @attr_reader value [true, false]
    class Boolean < ::CSVPlusPlus::Entities::Entity
      extend ::T::Sig

      sig { returns(::T::Boolean) }
      attr_reader :value

      sig { params(value: ::T.any(::String, ::T::Boolean)).void }
      # @param value [::String, boolean]
      def initialize(value)
        super()
        # TODO: probably can do a lot better in general on type validation
        @value = ::T.let(value.is_a?(::String) ? (value.downcase == 'true') : value, ::T::Boolean)
      end

      sig do
        override.params(_position: ::CSVPlusPlus::Runtime::Position).returns(::String)
      end
      # @param _position [Position]
      #
      # @return [::String]
      def evaluate(_position)
        @value.to_s.upcase
      end

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [::T::Boolean]
      def ==(other)
        case other
        when self.class
          value == other.value
        else
          false
        end
      end
    end
  end
end

# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A string value
    #
    # @attr_reader value [String]
    class String < ::CSVPlusPlus::Entities::Entity
      extend ::T::Sig

      sig { returns(::String) }
      attr_reader :value

      sig { params(value: ::String).void }
      # @param value [String] The string that has been parsed out of the template
      def initialize(value)
        super()

        @value = ::T.let(value.gsub(/^"|"$/, ''), ::String)
      end

      sig { override.params(_position: ::CSVPlusPlus::Runtime::Position).returns(::String) }
      # @param _position [Position]
      #
      # @return [::String]
      def evaluate(_position)
        "\"#{@value}\""
      end

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [BasicObject]
      #
      # @return [T::Boolean]
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

# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A string value
    #
    # @attr_reader value [String]
    class String < Entity
      extend ::T::Sig

      sig { returns(::String) }
      attr_reader :value

      sig { params(value: ::String).void }
      # @param value [String] The string that has been parsed out of the template
      def initialize(value)
        super(::CSVPlusPlus::Entities::Type::String)

        @value = ::T.let(value.gsub(/^"|"$/, ''), ::String)
      end

      sig { override.params(_runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      # @param _runtime [Runtime]
      #
      # @return [::String]
      def evaluate(_runtime)
        "\"#{@value}\""
      end

      sig { override.params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [T::Boolean]
      def ==(other)
        return false unless super

        other.is_a?(self.class) && @value == other.value
      end
    end
  end
end

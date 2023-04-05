# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # An entity that can take other entities as arguments.  Current use cases for this
    # are function calls and function definitions
    #
    # @attr_reader arguments [Array<Entity>] The arguments supplied to this entity.
    class EntityWithArguments < Entity
      extend ::T::Sig

      abstract!

      sig { returns(::T::Array[::CSVPlusPlus::Entities::Entity]) }
      attr_reader :arguments

      sig do
        params(
          type: ::CSVPlusPlus::Entities::Type,
          id: ::T.nilable(::Symbol),
          arguments: ::T::Array[::CSVPlusPlus::Entities::Entity]
        ).void
      end
      # @param type [Entities::Type]
      # @param id [::String]
      # @param arguments [Array<Entity>]
      def initialize(type, id: nil, arguments: [])
        super(type, id:)
        @arguments = arguments
      end

      sig { override.params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        return false unless other.is_a?(self.class)

        @arguments == other.arguments && super
      end

      protected

      sig do
        params(arguments: ::T::Array[::CSVPlusPlus::Entities::Entity])
          .returns(::T::Array[::CSVPlusPlus::Entities::Entity])
      end
      attr_writer :arguments

      sig { params(runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::T::Array[::String]) }
      def evaluate_arguments(runtime)
        @arguments.map { |arg| arg.evaluate(runtime) }
      end
    end
  end
end

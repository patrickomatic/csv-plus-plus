# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # An entity that can take other entities as arguments.  Current use cases for this
    # are function calls and function definitions
    #
    # @attr_reader arguments [Array<Entity>] The arguments supplied to this entity.
    class EntityWithArguments < ::CSVPlusPlus::Entities::Entity
      extend ::T::Sig
      extend ::T::Helpers
      extend ::T::Generic

      abstract!

      ArgumentsType = type_member
      public_constant :ArgumentsType

      sig { returns(::T::Array[::CSVPlusPlus::Entities::EntityWithArguments::ArgumentsType]) }
      attr_reader :arguments

      sig { params(arguments: ::T::Array[::CSVPlusPlus::Entities::EntityWithArguments::ArgumentsType]).void }
      # @param arguments [Array<ArgumentsType>]
      def initialize(arguments: [])
        super()
        @arguments = arguments
      end

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        case other
        when self.class
          @arguments == other.arguments
        else
          false
        end
      end
    end
  end
end

# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A basic building block of the abstract syntax tree (AST)
    #
    # @attr_reader id [Symbol] The identifier of the entity.  For functions this is the function name,
    #   for variables it's the variable name
    # @attr_reader type [Entities::Type] The type of the entity. Each type should have a corresponding class definition
    #   in CSVPlusPlus::Entities
    class Entity
      extend ::T::Sig
      extend ::T::Helpers

      abstract!

      sig { returns(::T.nilable(::Symbol)) }
      attr_reader :id

      sig { returns(::CSVPlusPlus::Entities::Type) }
      attr_reader :type

      sig { params(type: ::CSVPlusPlus::Entities::Type, id: ::T.nilable(::Symbol)).void }
      # @param type [Entities::Type]
      # @param id [Symbol, nil]
      def initialize(type, id: nil)
        @type = type
        @id = ::T.let(id&.downcase&.to_sym || nil, ::T.nilable(::Symbol))
      end

      sig { overridable.params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
      # Each class should define it's own version of #==
      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        self.class == other.class && @type == other.type && @id == other.id
      end

      sig { abstract.params(runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      # Uses the given +runtime+ to evaluate itself in the current context
      #
      # @param runtime [Runtime] The current runtime
      #
      # @return [::String]
      def evaluate(runtime); end
    end
  end
end

# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A function definition
    #
    # @attr_reader body [Entity] The body of the function.  +body+ can contain variable references
    #   from +@arguments+
    class Function < ::CSVPlusPlus::Entities::EntityWithArguments
      extend ::T::Sig
      include ::CSVPlusPlus::Entities::HasIdentifier

      ArgumentsType = type_member { { fixed: ::Symbol } }
      public_constant :ArgumentsType

      sig { returns(::Symbol) }
      attr_reader :id

      sig { returns(::CSVPlusPlus::Entities::Entity) }
      attr_reader :body

      sig { params(id: ::Symbol, arguments: ::T::Array[::Symbol], body: ::CSVPlusPlus::Entities::Entity).void }
      # @param id [Symbol] the name of the function - what it will be callable by
      # @param arguments [Array<Symbol>]
      # @param body [Entity]
      def initialize(id, arguments, body)
        super(arguments: arguments.map(&:to_sym))

        @body = ::T.let(body, ::CSVPlusPlus::Entities::Entity)
        @id = ::T.let(identifier(id), ::Symbol)
      end

      sig { override.params(position: ::CSVPlusPlus::Runtime::Position).returns(::String) }
      # @param position [Position]
      #
      # @return [String]
      def evaluate(position)
        "def #{@id.to_s.upcase}(#{arguments.map(&:to_s).join(', ')}) #{@body.evaluate(position)}"
      end

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [::T::Boolean]
      def ==(other)
        case other
        when self.class
          @body == other.body && super
        else
          false
        end
      end
    end
  end
end

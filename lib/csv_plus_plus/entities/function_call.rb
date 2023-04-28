# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A function call that can be either infix (A + B) or prefix (ADD(A, B))
    #
    # @attr_reader infix [boolean] Whether or not this function call is infix (X * Y, A + B, etc)
    class FunctionCall < EntityWithArguments
      extend ::T::Sig
      include ::CSVPlusPlus::Entities::HasIdentifier

      ArgumentsType = type_member { { fixed: ::CSVPlusPlus::Entities::Entity } }
      public_constant :ArgumentsType

      sig { returns(::T::Boolean) }
      attr_reader :infix

      sig { returns(::Symbol) }
      attr_reader :id

      sig do
        params(
          id: ::Symbol,
          arguments: ::T::Array[::CSVPlusPlus::Entities::FunctionCall::ArgumentsType],
          infix: ::T::Boolean
        ).void
      end
      # @param id [::String] The name of the function
      # @param arguments [Array<Entity>] The arguments to the function
      # @param infix [T::Boolean] Whether the function is infix
      def initialize(id, arguments, infix: false)
        super(arguments:)

        @id = ::T.let(identifier(id), ::Symbol)
        @infix = infix
      end

      sig { override.params(position: ::CSVPlusPlus::Runtime::Position).returns(::String) }
      # @param position [Position]
      #
      # @return [::String]
      def evaluate(position)
        evaluated_arguments = evaluate_arguments(position)

        if @infix
          "(#{evaluated_arguments.join(" #{@id} ")})"
        else
          "#{@id.to_s.upcase}(#{evaluated_arguments.join(', ')})"
        end
      end

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [BasicObject]
      #
      # @return [Boolean]
      def ==(other)
        case other
        when self.class
          @id == other.id && @infix == other.infix
        else
          false
        end
      end

      private

      sig { params(position: ::CSVPlusPlus::Runtime::Position).returns(::T::Array[::String]) }
      def evaluate_arguments(position)
        @arguments.map { |arg| arg.evaluate(position) }
      end
    end
  end
end

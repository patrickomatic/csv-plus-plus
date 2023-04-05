# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A function call that can be either infix (A + B) or prefix (ADD(A, B))
    #
    # @attr_reader infix [boolean] Whether or not this function call is infix (X * Y, A + B, etc)
    class FunctionCall < EntityWithArguments
      extend ::T::Sig

      sig { returns(::T::Boolean) }
      attr_reader :infix

      sig { params(id: ::String, arguments: ::T::Array[::CSVPlusPlus::Entities::Entity], infix: ::T::Boolean).void }
      # @param id [::String] The name of the function
      # @param arguments [Array<Entity>] The arguments to the function
      # @param infix [T::Boolean] Whether the function is infix
      def initialize(id, arguments, infix: false)
        super(::CSVPlusPlus::Entities::Type::FunctionCall, id:, arguments:)

        @infix = infix
      end

      sig { override.params(runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      # @param runtime [Runtime]
      #
      # @return [::String]
      def evaluate(runtime)
        evaluated_arguments = evaluate_arguments(runtime)

        if @infix
          "(#{evaluated_arguments.join(" #{@id} ")})"
        else
          "#{@id.to_s.upcase}(#{evaluated_arguments.join(', ')})"
        end
      end

      sig { override.params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        return false unless super

        other.is_a?(self.class) && @id == other.id && @infix == other.infix
      end
    end
  end
end

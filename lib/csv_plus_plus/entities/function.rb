# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A function definition
    #
    # @attr_reader body [Entity] The body of the function.  +body+ can contain variable references
    #   from +@arguments+
    class Function < EntityWithArguments
      extend ::T::Sig

      sig { returns(::CSVPlusPlus::Entities::Entity) }
      attr_reader :body

      sig { params(id: ::Symbol, arguments: ::T::Array[::Symbol], body: ::CSVPlusPlus::Entities::Entity).void }
      # @param id [Symbol] the name of the function - what it will be callable by
      # @param arguments [Array<Symbol>]
      # @param body [Entity]
      def initialize(id, arguments, body)
        super(::CSVPlusPlus::Entities::Type::Function, id:, arguments: arguments.map(&:to_sym))

        @body = ::T.let(body, ::CSVPlusPlus::Entities::Entity)
      end

      sig { override.params(runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      # @param runtime [Runtime]
      #
      # @return [::String]
      def evaluate(runtime)
        "def #{@id.to_s.upcase}(#{arguments.map(&:to_s).join(', ')}) #{@body.evaluate(runtime)}"
      end

      sig { override.params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [::T::Boolean]
      def ==(other)
        return false unless super

        other.is_a?(self.class) && @body == other.body
      end
    end
  end
end

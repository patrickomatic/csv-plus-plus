# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Entities
    # A basic building block of the abstract syntax tree (AST)
    #
    # @attr_reader id [Symbol] The identifier of the entity.  For functions this is the function name,
    #   for variables it's the variable name
    # @attr_reader type [Symbol] The type of the entity.  Valid values are defined in +::CSVPlusPlus::Entities::TYPES+
    class Entity
      extend ::T::Sig
      extend ::T::Helpers
      abstract!

      sig { returns(::T.nilable(::Symbol)) }
      attr_reader :id

      # TODO: this is really a union of type-specific symbols (:function, :var, :function_call, etc)
      sig { returns(::Symbol) }
      attr_reader :type

      sig { params(type: ::Symbol, id: ::T.nilable(::Symbol)).void }
      # @param type [::String, Symbol]
      # @param id [::String, nil]
      def initialize(type, id: nil)
        @type = type
        @id = ::T.let(id&.downcase&.to_sym || nil, ::T.nilable(::Symbol))
      end

      sig { params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        self.class == other.class && @type == other.type && @id == other.id
      end

      sig { params(method_name: ::String, _arguments: ::T.untyped).returns(::T::Boolean) }
      # Respond to predicates that correspond to types like #boolean?, #string?, etc
      #
      # @param method_name [Symbol] The +method_name+ to respond to
      # @param *_arguments [Any]
      def method_missing(method_name, *_arguments)
        if method_name =~ /^(\w+)\?$/
          t = ::Regexp.last_match(1)
          return super unless t

          a_type?(t) && @type == t.to_sym
        else
          super
        end
      end

      sig { params(method_name: ::String, _arguments: ::T.untyped).returns(::T::Boolean) }
      # Respond to predicates by type (entity.boolean?, entity.string?, etc)
      #
      # @param method_name [Symbol] The +method_name+ to respond to
      # @param *_arguments [Any]
      #
      # @return [boolean]
      def respond_to_missing?(method_name, *_arguments)
        (method_name =~ /^(\w+)\?$/ && a_type?(::T.must(::Regexp.last_match(1)))) || super
      end

      sig { abstract.params(_runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::String) }
      # Uses the given +runtime+ to evaluate itself in the current context
      #
      # @param runtime [Runtime] The current runtime
      #
      # @return [::String]
      def evaluate(_runtime); end

      private

      sig { params(str: ::String).returns(::T::Boolean) }
      def a_type?(str)
        ::CSVPlusPlus::Entities::TYPES.include?(str.to_sym)
      end
    end

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
        params(type: ::Symbol, id: ::T.nilable(::Symbol), arguments: ::T::Array[::CSVPlusPlus::Entities::Entity]).void
      end
      # @param type [::String, Symbol]
      # @param id [::String]
      # @param arguments [Array<Entity>]
      def initialize(type, id: nil, arguments: [])
        super(type, id:)
        @arguments = arguments
      end

      sig { params(other: ::CSVPlusPlus::Entities::Entity).returns(::T::Boolean) }
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

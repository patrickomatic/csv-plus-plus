# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # References in an AST that need to be resolved
    #
    # @attr functions [Array<Entities::Function>] Functions references
    # @attr variables [Array<Entities::Variable>] Variable references
    class References
      extend ::T::Sig

      sig { returns(::T::Array[::CSVPlusPlus::Entities::FunctionCall]) }
      attr_accessor :functions

      sig { returns(::T::Array[::CSVPlusPlus::Entities::Variable]) }
      attr_accessor :variables

      sig do
        params(
          ast: ::CSVPlusPlus::Entities::Entity,
          position: ::CSVPlusPlus::Runtime::Position,
          scope: ::CSVPlusPlus::Runtime::Scope
        ).returns(::CSVPlusPlus::Runtime::References)
      end
      # Extract references from an AST and return them in a new +References+ object
      #
      # @param ast [Entity] An +Entity+ to do a depth first search on for references.  Entities can be
      #   infinitely deep because they can contain other function calls as params to a function call
      # @param scope [Scope] The current scope
      #
      # @return [References]
      def self.extract(ast, position, scope)
        new.tap do |refs|
          ::CSVPlusPlus::Runtime::Graph.depth_first_search(ast) do |node|
            unless node.is_a?(::CSVPlusPlus::Entities::FunctionCall) || node.is_a?(::CSVPlusPlus::Entities::Variable)
              next
            end

            refs.functions << node if function_reference?(node, scope)
            refs.variables << node if variable_reference?(node, position, scope)
          end
        end
      end

      sig do
        params(
          node: ::CSVPlusPlus::Entities::Entity,
          position: ::CSVPlusPlus::Runtime::Position,
          scope: ::CSVPlusPlus::Runtime::Scope
        ).returns(::T::Boolean)
      end
      # Is the node a resolvable variable reference?
      #
      # @param node [Entity] The node to check if it's resolvable
      # @param scope [Scope] The current scope
      #
      # @return [boolean]
      def self.variable_reference?(node, position, scope)
        return false unless node.is_a?(::CSVPlusPlus::Entities::Variable)

        return true if scope.in_scope?(node.id, position)

        raise(
          ::CSVPlusPlus::Error::ModifierSyntaxError.new(
            "#{node.id} can only be referenced within the ![[expand]] where it was defined.",
            bad_input: node.id.to_s
          )
        )
      end
      private_class_method :variable_reference?

      sig do
        params(node: ::CSVPlusPlus::Entities::Entity, scope: ::CSVPlusPlus::Runtime::Scope).returns(::T::Boolean)
      end
      # Is the node a resolvable function reference?
      #
      # @param node [Entity] The node to check if it's resolvable
      # @param scope [Scope] The current scope
      #
      # @return [boolean]
      def self.function_reference?(node, scope)
        node.is_a?(::CSVPlusPlus::Entities::FunctionCall) \
          && (scope.functions.key?(node.id) || ::CSVPlusPlus::Entities::Builtins.builtin_function?(node.id))
      end
      private_class_method :function_reference?

      sig { void }
      # Create an object with empty references.  The caller will build them up as it depth-first-searches
      def initialize
        @functions = ::T.let([], ::T::Array[::CSVPlusPlus::Entities::FunctionCall])
        @variables = ::T.let([], ::T::Array[::CSVPlusPlus::Entities::Variable])
      end

      sig { params(other: ::CSVPlusPlus::Runtime::References).returns(::T::Boolean) }
      # @param other [References]
      #
      # @return [boolean]
      def ==(other)
        @functions == other.functions && @variables == other.variables
      end

      sig { returns(::T::Boolean) }
      # Are there any references to be resolved?
      #
      # @return [::T::Boolean]
      def empty?
        @functions.empty? && @variables.empty?
      end
    end
  end
end

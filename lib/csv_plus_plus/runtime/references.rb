# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # References in an AST that need to be resolved
    #
    # @attr functions [Array<Entities::Function>] Functions references
    # @attr variables [Array<Entities::Variable>] Variable references
    # TODO: turn this into a CanExtractReferences?
    class References
      extend ::T::Sig

      sig { returns(::T::Array[::CSVPlusPlus::Entities::FunctionCall]) }
      attr_accessor :functions

      sig { returns(::T::Array[::CSVPlusPlus::Entities::Variable]) }
      attr_accessor :variables

      sig do
        params(
          ast: ::CSVPlusPlus::Entities::Entity,
          runtime: ::CSVPlusPlus::Runtime::Runtime
        ).returns(::CSVPlusPlus::Runtime::References)
      end
      # Extract references from an AST and return them in a new +References+ object
      #
      # @param ast [Entity] An +Entity+ to do a depth first search on for references.  Entities can be
      #   infinitely deep because they can contain other function calls as params to a function call
      # @param runtime [Runtime] The current runtime
      #
      # @return [References]
      def self.extract(ast, runtime)
        new.tap do |refs|
          ::CSVPlusPlus::Runtime::Graph.depth_first_search(ast) do |node|
            unless node.type == ::CSVPlusPlus::Entities::Type::FunctionCall \
                || node.type == ::CSVPlusPlus::Entities::Type::Variable

              next
            end

            refs.functions << node if function_reference?(node, runtime)
            refs.variables << node if variable_reference?(node, runtime)
          end
        end
      end

      sig do
        params(node: ::CSVPlusPlus::Entities::Entity, runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::T::Boolean)
      end
      # Is the node a resolvable variable reference?
      #
      # @param node [Entity] The node to check if it's resolvable
      # @param runtime [Runtime] The current runtime
      #
      # @return [boolean]
      def self.variable_reference?(node, runtime)
        return false unless node.type == ::CSVPlusPlus::Entities::Type::Variable

        if runtime.in_scope?(node.id)
          true
        else
          runtime.raise_modifier_syntax_error(
            "#{node.id} can only be referenced within the ![[expand]] where it was defined.",
            node.id.to_s
          )
        end
      end
      private_class_method :variable_reference?

      sig do
        params(node: ::CSVPlusPlus::Entities::Entity, runtime: ::CSVPlusPlus::Runtime::Runtime).returns(::T::Boolean)
      end
      # Is the node a resolvable function reference?
      #
      # @param node [Entity] The node to check if it's resolvable
      # @param runtime [Runtime] The current runtime
      #
      # @return [boolean]
      def self.function_reference?(node, runtime)
        node.type == ::CSVPlusPlus::Entities::Type::FunctionCall \
          && (runtime.defined_function?(node.id) || runtime.builtin_function?(::T.must(node.id)))
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

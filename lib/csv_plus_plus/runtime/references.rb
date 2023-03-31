# typed: false
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # References in an AST that need to be resolved
    #
    # @attr functions [Array<Entities::Function>] Functions references
    # @attr variables [Array<Entities::Variable>] Variable references
    class References
      # TODO: turn this into a CanExtractReferences?
      attr_accessor :functions, :variables

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
            next unless node.function_call? || node.variable?

            refs.functions << node if function_reference?(node, runtime)
            refs.variables << node if variable_reference?(node, runtime)
          end
        end
      end

      # Is the node a resolvable variable reference?
      #
      # @param node [Entity] The node to check if it's resolvable
      # @param runtime [Runtime] The current runtime
      #
      # @return [boolean]
      def self.variable_reference?(node, runtime)
        return false unless node.variable?

        if runtime.in_scope?(node.id)
          true
        else
          runtime.raise_modifier_syntax_error(
            bad_input: node,
            message: "#{var_id} can only be referenced within the ![[expand]] where it was defined."
          )
        end
      end
      private_class_method :variable_reference?

      # Is the node a resolvable function reference?
      #
      # @param node [Entity] The node to check if it's resolvable
      # @param runtime [Runtime] The current runtime
      #
      # @return [boolean]
      def self.function_reference?(node, runtime)
        node.function_call? && (runtime.defined_function?(node.id) || runtime.builtin_function?(node.id))
      end
      private_class_method :function_reference?

      # Create an object with empty references.  The caller will build them up as it depth-first-searches
      def initialize
        @functions = []
        @variables = []
      end

      # @param other [References]
      #
      # @return [boolean]
      def ==(other)
        @functions == other.functions && @variables == other.variables
      end

      # Are there any references to be resolved?
      #
      # @return [boolean]
      def empty?
        @functions.empty? && @variables.empty?
      end
    end
  end
end

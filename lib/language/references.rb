# frozen_string_literal: true

module CSVPlusPlus
  module Language
    # References in an AST that need to be resolved
    class References
      attr_accessor :functions, :variables

      # Extract references from an AST.  And return them in a new +References+ object
      def self.extract(ast, code_section)
        new.tap do |refs|
          ::CSVPlusPlus::Graph.depth_first_search(ast) do |node|
            next unless node.function_call? || node.variable?

            refs.functions << node if function_reference?(node, code_section)
            refs.variables << node if node.variable?
          end
        end
      end

      # Is the node a resolvable reference?
      def self.function_reference?(node, code_section)
        node.function_call? && (code_section.defined_function?(node.id) || ::BUILTIN_FUNCTIONS.key?(node.id))
      end

      private_class_method :function_reference?

      # Create an object with empty references.  The caller will build them up as it depth-first-searches
      def initialize
        @functions = []
        @variables = []
      end

      # are there any references to be resolved?
      def empty?
        @functions.empty? && @variables.empty?
      end

      # ==
      def ==(other)
        super && @functions == other.functions && @variables == other.variables
      end
    end
  end
end

# frozen_string_literal: true

require 'tsort'

module CSVPlusPlus
  module Runtime
    # Graph ordering and searching functions
    module Graph
      # Get a list of all variables references in a given +ast+
      # TODO: this is only used in one place - refactor it
      def self.variable_references(ast, runtime, include_runtime_variables: false)
        depth_first_search(ast) do |node|
          next unless node.variable?

          node.id if !runtime.builtin_variable?(node.id) || include_runtime_variables
        end
      end

      # Create a dependency graph of +variables+
      def self.dependency_graph(variables, runtime)
        ::CSVPlusPlus::Runtime::Graph::DependencyGraph[
          variables.map { |var_id, ast| [var_id, variable_references(ast, runtime)] }
        ]
      end

      # TODO: I don't think we use this anymore - it was useful when I wanted to resolve variables in their dependency
      #   order
      #
      # Perform a topological sort on a +DependencyGraph+.  A toplogical sort is noteworthy
      # because it will give us the order in which we need to resolve our variable dependencies.
      #
      # Given this dependency graph:
      #
      #  { a: [b c], b: [c], c: [d], d: [] }
      #
      # it will return:
      #
      #  [d, c, b, a]
      #
      def self.topological_sort(dependencies)
        dependencies.tsort
      end

      # Do a DFS on an AST starting at +node+
      def self.depth_first_search(node, accum = [], &)
        ret = yield(node)
        accum << ret unless ret.nil?

        return accum unless node.function_call?

        node.arguments.each { |n| depth_first_search(n, accum, &) }
        accum
      end

      # A dependency graph represented as a +Hash+ which will be used by our +topological_sort+ function
      class DependencyGraph < Hash
        include ::TSort
        alias tsort_each_node each_key

        # sort each child
        def tsort_each_child(node, &)
          fetch(node).each(&)
        end
      end
    end
  end
end

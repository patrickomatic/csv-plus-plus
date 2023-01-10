require 'tsort'
require_relative 'syntax_error'

module CSVPlusPlus
  module Language
    END_OF_CODE_SECTION = "---"
    VARIABLE_REF = "$$"
    class AST

      def self.variable_references(ast)
        depth_first_search ast do |node|
          node.id if node.type == :variable
        end
      end

      # TODO we need to exclude runtime-vars from being included in here
      def self.resolve_variables(variables, execution_context) 
        # we have a hash of variables => ASTs but they might have references to each other, so 
        # we need to interpolate them first (before interpolating the cell values)
        var_dependencies  = GraphHash[
          variables.map {|var, ast| [var, variable_references(ast)]}
        ]

        # are there any references that we don't have variables for? (aka undefined variable)
        unbound_vars = var_dependencies.values.flatten - variables.keys
        if unbound_vars.length > 0
          raise SyntaxError.new("Undefined variables", unbound_vars.join(', '), execution_context)
        end

        resolved_vars = {}
        begin
          # a topological sort will give us the order of dependencies
          resolution_order = topological_sort var_dependencies

          # for each var and each dependency it has, build up and mutate resolved_vars
          resolution_order.each do |var|
            resolved_vars[var] = variables[var].dup

            var_dependencies[var].each do |dependency|
              resolved_vars[var] = resolve_variable(
                resolved_vars[var],
                dependency,
                variables[dependency]
              )
            end
          end
        rescue TSort::Cyclic
          raise SyntaxError.new("Cyclic variable dependency detected", var_refs.keys, execution_context)
        end

        resolved_vars
      end

      def self.resolve_variable(ast, var_id, value)
        copy_tree(ast) {|node| node.type == :variable && node.id == var_id ? value : node}
      end

      def self.copy_tree(node, &block)
        # each part of the node recursively handles dup 
        node.dup
      end

      def self.depth_first_search(node, accum = [], &block)
        ret = yield node
        accum << ret unless ret.nil?

        if node.type == :function || node.type == :function_call
          node.arguments.each {|n| depth_first_search(n, accum, &block)}
        end

        accum
      end

      def self.topological_sort(graph)
        graph.tsort
      end

      protected

      class GraphHash < Hash
        include TSort
        alias tsort_each_node each_key
        def tsort_each_child(node, &block)
          fetch(node).each(&block)
        end
      end
    end
  end
end

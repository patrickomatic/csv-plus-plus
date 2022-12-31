require 'tsort'
require_relative 'syntax_error'

module CSVPlusPlus
  class AST
    END_OF_CODE_SECTION = "---"
    VARIABLE_REF = "$$"

    def self.variable_references(ast)
      depth_first_search ast do |node|
        type, value = node
        value if type == :var
      end
    end

    def self.interpolate_variables(ast, variables) 
      # we have a hash of variables => ASTs but they might have references to each other, so 
      # we need to interpolate them first (before interpolating the cell values)
      var_dependencies  = GraphHash[variables.map {|k, v| [k, variable_references(v)]}]

      # are there any references that we don't have variables for? (aka undefined variable)
      unbound_vars = var_dependencies.values.flatten - variables.keys
      if unbound_vars.length > 0
        raise SyntaxError.new("Undefined variables", unbound_vars.join(', '))
      end

      interpolated_vars = {}
      begin
        # a topological sort will figure out the order of dependencies
        resolution_order = topological_sort var_dependencies

        # for each var and each dependency it has, build up and mutate interpolated_vars
        resolution_order.each do |var|
          interpolated_vars[var] = variables[var].dup

          var_dependencies[var].each do |dependency|
            interpolated_vars[var] = interpolate_variable(
              interpolated_vars[var],
              dependency,
              variables[dependency]
            )
          end
        end
      rescue TSort::Cyclic
        raise SyntaxError.new("Cyclic variable dependency detected", var_refs.keys)
      end

      # now just mutate ast with our interpolated_vars
      transformed_ast = ast
      interpolated_vars.each do |var, value|
        transformed_ast = interpolate_variable(transformed_ast, var, value)
      end
      transformed_ast
    end

    def self.interpolate_variable(ast, var, value)
      copy_tree(ast) {|node| node[0] == :var && node[1] == var ? value : node}
    end

    def self.copy_tree(ast, &block)
      return (yield ast).dup if ast.first.is_a? Symbol
      node, rest = ast
      ret = (yield node).dup
      [ret, rest.map {|r| copy_tree(r, &block)}]
    end

    def self.depth_first_search(ast, accum = [], &block)
      yield_and_accum = -> (y) do
        ret = yield y
        accum << ret unless ret.nil?
      end

      if ast.first.is_a? Symbol
        yield_and_accum.call ast
      else 
        node, rest = ast

        yield_and_accum.call node
        rest.each do |r| 
          depth_first_search(r, accum, &block)
        end
        yield_and_accum.call [:after_fn]
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

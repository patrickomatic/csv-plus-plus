require_relative './entities'
require 'tsort'
require_relative './syntax_error'

module CSVPlusPlus
  module Language
    END_OF_CODE_SECTION = "---"
    VARIABLE_REF = "$$"

    protected

    RUNTIME_VARIABLES = {
      rownum: RuntimeValue.new(-> (ec) { Number.new(ec.row_index + 1) }),
      cellnum: RuntimeValue.new(-> (ec) { Number.new(ec.cell_index + 1) }),
    }

    BUILTIN_FUNCTIONS = {
      # TODO not sure we need this...
      # =REF(C) === =INDIRECT($$C)
=begin
      ref: -> (args, ec) {
        Function.new(:ref,
                     [Variable.new(:cell)],
                     FunctionCall.new(:indirect,
                                      [Variable.new(:cell)]))
      }

      # =CELLREF(C) === =INDIRECT(CONCAT($$C, $$rownum))
      cellref: -> (args, ec) {
        Function.new(:cellref,
                     [Variable.new(:cell)],
                     FunctionCall.new(:indirect,
                                      [FunctionCall.new(:concat, [Variable.new(:cell), Variable.new(:rownum)])]))
      }
=end

      sheetref: -> (args, ec) {
        # TODO not quite sure how this will be used yet
        raise "not implemented"
      }
    }

    public

    class GlobalScope
      def initialize(code_section)
        @code_section = code_section
      end

      def resolve_cell_value(execution_context)
        ast = execution_context.cell.ast
        return nil if ast.nil?

        variables_referenced = variable_references(ast, include_runtime: true)
        variables_referenced.reduce(ast.dup) do |resolved_ast, var|
          value = (
            if @code_section.variables.has_key?(var)
              @code_section.variables[var]
            elsif RUNTIME_VARIABLES.has_key?(var)
              RUNTIME_VARIABLES[var].resolve_fn.call(execution_context)
            else
              raise SyntaxError.new("Undefined variable reference", var, execution_context)
            end)

          resolve_variable(resolved_ast, var, value)
        end
      end

      # XXX this is weird because we already have a reference to code_section
      def resolve_static_variables(variables, execution_context)
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

      def resolve_variable(ast, var_id, value)
        copy_tree_with_replacement(ast, var_id, value)
      end

      def variable_references(ast, include_runtime: false)
        depth_first_search ast do |node|
          if node.type == :variable && (!RUNTIME_VARIABLES.has_key?(node.id) || include_runtime)
            node.id
          end
        end
      end

      def copy_tree_with_replacement(node, var_id, replacement)
        if node.type == :function_call
          arguments = node.arguments.map {|n| copy_tree_with_replacement(n, var_id, replacement)}
          FunctionCall.new(node.id, arguments)
        else
          if node.type == :variable && node.id == var_id
            replacement
          else
            node
          end
        end
      end

      def depth_first_search(node, accum = [], &block)
        ret = yield node
        accum << ret unless ret.nil?

        if node.type == :function || node.type == :function_call
          node.arguments.each {|n| depth_first_search(n, accum, &block)}
        end

        accum
      end

      def topological_sort(graph)
        graph.tsort
      end

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

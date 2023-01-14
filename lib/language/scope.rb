# frozen_string_literal: true

require_relative '../code_section'
require_relative '../graph'
require_relative './entities'
require_relative './syntax_error'

BUILTIN_FUNCTIONS = {
  # TODO: not sure we need this...
  # =REF(C) === =INDIRECT($$C)
  #       ref: -> (args, runtime) {
  #         Function.new(:ref,
  #                      [Variable.new(:cell)],
  #                      FunctionCall.new(:indirect,
  #                                       [Variable.new(:cell)]))
  #       }
  #
  #       # =CELLREF(C) === =INDIRECT(CONCAT($$C, $$rownum))
  #       cellref: -> (args, runtime) {
  #         Function.new(:cellref,
  #                      [Variable.new(:cell)],
  #                      FunctionCall.new(:indirect,
  #                                       [FunctionCall.new(:concat,
  #                                       [Variable.new(:cell), Variable.new(:rownum)])]))
  #       }

  # sheetref: lambda { |args, runtime|
  # }
}.freeze

module CSVPlusPlus
  module Language
    ##
    # A class representing the scope of the current Template and responsible for
    # resolving variables
    class Scope
      attr_accessor :code_section

      # initialize with a CodeSection
      def initialize(code_section = nil)
        @code_section = code_section || ::CSVPlusPlus::CodeSection.new
      end

      # to_s
      def to_s
        "Scope(code_section: #{@code_section})"
      end

      # Resolve all values in the ast of the current cell being processed
      def resolve_cell_value(runtime)
        ast = runtime.cell.ast
        return if ast.nil?

        variables_referenced = ::CSVPlusPlus::Graph.variable_references(ast, runtime, include_runtime_variables: true)
        variables_referenced.reduce(ast.dup) do |resolved_ast, var|
          resolve_variable(resolved_ast, var, resolve(var, runtime))
        end
      end

      # Resolve all variables references defined statically in the code section
      def resolve_static_variables!(runtime)
        variables = @code_section.variables
        var_dependencies, resolution_order = variable_resolution_order(variables, runtime)
        resolve_dependencies(var_dependencies, resolution_order, variables)
      end

      # Resolve a single variable in a given +ast+
      def resolve_variable(ast, var_id, value)
        copy_tree_with_replacement(ast, var_id, value)
      end

      # Make a copy of the AST represented by +node+ and replace +var_id+ with +replacement+ as we go
      def copy_tree_with_replacement(node, var_id, replacement)
        if node.type == :function_call
          arguments = node.arguments.map { |n| copy_tree_with_replacement(n, var_id, replacement) }
          ::CSVPlusPlus::Language::FunctionCall.new(node.id, arguments)
        elsif node.type == :variable && node.id == var_id
          replacement
        else
          node
        end
      end

      private

      def resolve(var_id, runtime)
        return @code_section.variables[var_id] if @code_section.variables.key?(var_id)

        # this will throw a syntax error if it doesn't exist (which is what we want)
        runtime.runtime_value(var_id)
      end

      def check_unbound_vars(dependencies, variables, runtime)
        unbound_vars = dependencies.values.flatten - variables.keys
        return if unbound_vars.empty?

        runtime.raise_syntax_error('Undefined variables', unbound_vars.map(&:to_s).join(', '))
      end

      def variable_resolution_order(variables, runtime)
        # we have a hash of variables => ASTs but they might have references to each other, so
        # we need to interpolate them first (before interpolating the cell values)
        var_dependencies = ::CSVPlusPlus::Graph.dependency_graph(variables, runtime)
        # are there any references that we don't have variables for? (undefined variable)
        check_unbound_vars(var_dependencies, variables, runtime)

        # a topological sort will give us the order of dependencies
        [var_dependencies, ::CSVPlusPlus::Graph.topological_sort(var_dependencies)]
        # TODO: don't expose this exception directly to the caller
      rescue ::TSort::Cyclic
        runtime.raise_syntax_error('Cyclic variable dependency detected', var_refs.keys)
      end

      def resolve_dependencies(var_dependencies, resolution_order, variables)
        resolved_vars = {}

        # for each var and each dependency it has, build up and mutate resolved_vars
        resolution_order.each do |var|
          resolved_vars[var] = variables[var].dup

          var_dependencies[var].each do |dependency|
            resolved_vars[var] = resolve_variable(resolved_vars[var], dependency, variables[dependency])
          end
        end

        resolved_vars
      end
    end
  end
end

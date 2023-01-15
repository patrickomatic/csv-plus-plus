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
    # A class representing the scope of the current Template and responsible for resolving variables
    class Scope
      attr_reader :code_section, :runtime

      # initialize with a +Runtime+ and optional +CodeSection+
      def initialize(runtime:, code_section: nil)
        @code_section = code_section if code_section
        @runtime = runtime
      end

      # Resolve all values in the ast of the current cell being processed
      # rubocop:disable Metrics/CyclomaticComplexity
      def resolve_cell_value
        return unless (ast = @runtime.cell&.ast)

        references_to_resolve = cell_references_to_resolve(ast)
        var_references_to_resolve = references_to_resolve.filter(&:variable?)
        fn_references_to_resolve = references_to_resolve.filter(&:function_call?)

        ast =
          var_references_to_resolve.reduce(ast.dup) do |resolved_ast, var|
            variable_replace(resolved_ast, var.id, resolve(var.id))
          end

        fn_references_to_resolve.reduce(ast) { |acc, elem| acc if elem }
      end
      # rubocop:enable Metrics/CyclomaticComplexity

      # Set the +code_section+ and resolve all inner dependencies in it's @variables and @functions.
      def code_section=(code_section)
        @code_section = code_section

        resolve_static_variables!
        resolve_static_functions!
      end

      # to_s
      def to_s
        "Scope(code_section: #{@code_section}, runtime: #{@runtime})"
      end

      private

      def cell_references_to_resolve(ast)
        ::CSVPlusPlus::Graph.depth_first_search(ast) do |node|
          # functions that are defined and variablesj
          next node if node.variable? || (node.function_call? && @code_section.function_defined?(node.name))
        end
      end

      # Resolve all variable references defined statically in the code section
      def resolve_static_variables!
        variables = @code_section.variables
        var_dependencies, resolution_order = variable_resolution_order(variables)
        resolve_dependencies(var_dependencies, resolution_order, variables)
      end

      # Resolve all functions defined statically in the code section
      def resolve_static_functions!
        # functions = @code_section.functions
        # resolution_order = variable_resolution_order(variables, runtime)
        # TODO
        #
      end

      # Make a copy of the AST represented by +node+ and replace +fn_id+ with +replacement+ throughout
      def function_replace(node, fn_id, replacement)
        if node.function_call? && node.name == fn_id
          # TODO
          node.id
        elsif node.function_call?
          arguments = node.arguments.map { |n| function_replace(n, fn_id, replacement) }
          ::CSVPlusPlus::Language::FunctionCall.new(node.id, arguments)
        else
          node
        end
      end

      # Make a copy of the AST represented by +node+ and replace +var_id+ with +replacement+ throughout
      def variable_replace(node, var_id, replacement)
        if node.function_call?
          arguments = node.arguments.map { |n| variable_replace(n, var_id, replacement) }
          ::CSVPlusPlus::Language::FunctionCall.new(node.id, arguments)
        elsif node.variable? && node.id == var_id
          replacement
        else
          node
        end
      end

      def resolve(var_id)
        return @code_section.variables[var_id.to_sym] if @code_section.variables.key?(var_id.to_sym)

        # this will throw a syntax error if it doesn't exist (which is what we want)
        @runtime.runtime_value(var_id)
      end

      def check_unbound_vars(dependencies, variables)
        unbound_vars = dependencies.values.flatten - variables.keys
        return if unbound_vars.empty?

        @runtime.raise_syntax_error('Undefined variables', unbound_vars.map(&:to_s).join(', '))
      end

      def variable_resolution_order(variables)
        # we have a hash of variables => ASTs but they might have references to each other, so
        # we need to interpolate them first (before interpolating the cell values)
        var_dependencies = ::CSVPlusPlus::Graph.dependency_graph(variables, @runtime)
        # are there any references that we don't have variables for? (undefined variable)
        check_unbound_vars(var_dependencies, variables)

        # a topological sort will give us the order of dependencies
        [var_dependencies, ::CSVPlusPlus::Graph.topological_sort(var_dependencies)]
        # TODO: don't expose this exception directly to the caller
      rescue ::TSort::Cyclic
        @runtime.raise_syntax_error('Cyclic variable dependency detected', var_refs.keys)
      end

      def resolve_dependencies(var_dependencies, resolution_order, variables)
        resolved_vars = {}

        # for each var and each dependency it has, build up and mutate resolved_vars
        resolution_order.each do |var|
          resolved_vars[var] = variables[var].dup

          var_dependencies[var].each do |dependency|
            resolved_vars[var] = variable_replace(resolved_vars[var], dependency, variables[dependency])
          end
        end

        resolved_vars
      end
    end
  end
end

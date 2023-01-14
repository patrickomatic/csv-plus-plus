# frozen_string_literal: true

require_relative '../code_section'
require_relative './entities'
require 'tsort'
require_relative './syntax_error'

LANG = ::CSVPlusPlus::Language

module CSVPlusPlus
  module Language
    END_OF_CODE_SECTION = '---'
    public_constant :END_OF_CODE_SECTION

    VARIABLE_REF = '$$'
    public_constant :VARIABLE_REF

    # A class that can be sorted by #topological_sort
    class GraphHash < Hash
      include ::TSort
      alias tsort_each_node each_key

      # sort each child
      def tsort_each_child(node, &)
        fetch(node).each(&)
      end
    end

    ##
    # A class representing the scope of the current Template and responsible for
    # resolving variables
    class Scope
      RUNTIME_VARIABLES = {
        rownum: ::LANG::RuntimeValue.new(->(r) { ::LANG::Number.new(r.row_index + 1) }),
        cellnum: ::LANG::RuntimeValue.new(->(r) { ::LANG::Number.new(r.cell_index + 1) })
      }.freeze
      # XXX why doesn't this work as a private??
      public_constant :RUNTIME_VARIABLES

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
      private_constant :BUILTIN_FUNCTIONS

      attr_accessor :code_section

      # initialize with a CodeSection
      def initialize(code_section = nil)
        @code_section = code_section || ::CSVPlusPlus::CodeSection.new
      end

      # Resolve all values in the ast of the current cell being processed
      def resolve_cell_value(runtime)
        ast = runtime.cell.ast
        return if ast.nil?

        variables_referenced = variable_references(ast, include_runtime: true)
        variables_referenced.reduce(ast.dup) do |resolved_ast, var|
          resolve_variable(resolved_ast, var, resolve_to(var, runtime))
        end
      end

      # XXX this is weird because we already have a reference to code_section
      # Resolve all variables references defined statically in the code section
      # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
      def resolve_static_variables(variables, runtime)
        # we have a hash of variables => ASTs but they might have references to each other, so
        # we need to interpolate them first (before interpolating the cell values)
        var_dependencies = ::CSVPlusPlus::Language::GraphHash[
          variables.map do |var, ast|
            [var, variable_references(ast)]
          end
        ]

        check_unbound_vars(var_dependencies, variables, runtime)

        # are there any references that we don't have variables for? (aka undefined variable)
        resolved_vars = {}
        begin
          # a topological sort will give us the order of dependencies
          resolution_order = topological_sort(var_dependencies)

          # for each var and each dependency it has, build up and mutate resolved_vars
          resolution_order.each do |var|
            resolved_vars[var] = variables[var].dup

            var_dependencies[var].each do |dependency|
              resolved_vars[var] = resolve_variable(resolved_vars[var], dependency, variables[dependency])
            end
          end
        rescue ::TSort::Cyclic
          runtime.raise_syntax_error('Cyclic variable dependency detected', var_refs.keys)
        end

        resolved_vars
      end
      # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

      # Resolve a single variable in a given +ast+
      def resolve_variable(ast, var_id, value)
        copy_tree_with_replacement(ast, var_id, value)
      end

      # Get a list of all variables references in a given +ast+
      def variable_references(ast, include_runtime: false)
        depth_first_search(ast) do |node|
          next unless node.type == :variable

          node.id if !runtime_variable(node.id) || include_runtime
        end
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

      # Do a DFS on an AST starting at +node+
      def depth_first_search(node, accum = [], &)
        ret = yield(node)
        accum << ret unless ret.nil?

        return accum unless node.function?

        node.arguments.each { |n| depth_first_search(n, accum, &) }
        accum
      end

      # Perform a topological (dependency-resolution) sort
      def topological_sort(graph)
        graph.tsort
      end

      # to_s
      def to_s
        'Scope(TODO)'
      end

      private

      def runtime_variable(id)
        self.class::RUNTIME_VARIABLES[id]
      end

      def resolve_to(var_id, runtime)
        if @code_section.variables.key?(var_id)
          @code_section.variables[var_id]
        elsif (runtime_var = runtime_variable(var_id))
          runtime_var.resolve_fn.call(runtime)
        else
          runtime.raise_syntax_error('Undefined variable reference', var_id)
        end
      end

      def check_unbound_vars(dependencies, variables, runtime)
        unbound_vars = dependencies.values.flatten - variables.keys
        return if unbound_vars.empty?

        runtime.raise_syntax_error('Undefined variables', unbound_vars.map(&:to_s).join(', '))
      end
    end
  end
end

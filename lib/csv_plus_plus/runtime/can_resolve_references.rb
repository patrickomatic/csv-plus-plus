# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # Methods for resolving functions and variables.  These should be included onto a class that has +@variables+ and
    # +@functions+ instance variables.
    module CanResolveReferences
      # Resolve all values in the ast of the current cell being processed
      #
      # @return [Entity]
      def resolve_cell_value
        return unless (ast = @cell&.ast)

        last_round = nil
        loop do
          refs = ::CSVPlusPlus::Runtime::References.extract(ast, self)
          return ast if refs.empty?

          # TODO: throw an error here instead I think - basically we did a round and didn't make progress
          return ast if last_round == refs

          ast = resolve_functions(resolve_variables(ast, refs.variables), refs.functions)
        end
      end

      # Bind +var_id+ to the current cell
      #
      # @param var_id [Symbol] The name of the variable to bind the cell reference to
      #
      # @return [CellReference]
      def bind_variable_to_cell(var_id)
        def_variable(
          var_id,
          ::CSVPlusPlus::Entities::CellReference.new(
            cell_index: @cell_index,
            row_index: @row_index
          )
        )
      end

      # Bind +var_id+ relative to an ![[expand]] modifier.
      #
      # @param var_id [Symbol] The name of the variable to bind the cell reference to
      # @param expand [Expand] The expand where the variable is accessible (where it will be bound relative to)
      #
      # @return [CellReference]
      def bind_variable_in_expand(var_id, expand)
        def_variable(
          var_id,
          ::CSVPlusPlus::Entities::CellReference.new(
            scoped_to_expand: expand,
            cell_index: @cell_index
          )
        )
      end

      # Variables outside of an ![[expand=...] are always in scope.  If it's defined within an expand then things
      # get trickier because the variable is only in scope while we're processing cells within that expand.
      #
      # @param var_id [Symbol] The variable's identifier that we are checking if it's in scope
      #
      # @return [boolean]
      def in_scope?(var_id)
        value = @variables[var_id]

        raise_modifier_syntax_error('Undefined variable reference', var_id.to_s) if value.nil?

        expand = value.cell_reference? && value.scoped_to_expand
        return true unless expand

        unless expand.starts_at
          raise(::CSVPlusPlus::Error::Error, 'Must call Template.expand_rows! before checking the scope of expands.')
        end

        @row_index >= expand.starts_at && (expand.ends_at.nil? || row_index <= expand.ends_at)
      end

      private

      # Resolve all variable references defined statically in the code section
      #     def resolve_static_variables!
      #       last_var_dependencies = {}
      #       loop do
      #         var_dependencies, resolution_order = variable_resolution_order(only_static_vars(variables))
      #         return if var_dependencies == last_var_dependencies
      #
      #         # TODO: make the contract better here
      #         @variables = resolve_dependencies(var_dependencies, resolution_order, variables)
      #         last_var_dependencies = var_dependencies.clone
      #       end
      #     end
      #
      #     def only_static_vars(var_dependencies)
      #       var_dependencies.reject { |k| @runtime.builtin_variable?(k) }
      #     end

      def resolve_functions(ast, refs)
        refs.reduce(ast.dup) do |acc, elem|
          function_replace(acc, elem.id, resolve_function(elem.id))
        end
      end

      def resolve_variables(ast, refs)
        refs.reduce(ast.dup) do |acc, elem|
          variable_replace(acc, elem.id, resolve_variable(elem.id))
        end
      end

      # Make a copy of the AST represented by +node+ and replace +fn_id+ with +replacement+ throughout
      # rubocop:disable Metrics/MethodLength
      def function_replace(node, fn_id, replacement)
        if node.function_call? && node.id == fn_id
          call_function_or_builtin(replacement, node)
        elsif node.function_call?
          # not our function, but continue our depth first search on it
          ::CSVPlusPlus::Entities::FunctionCall.new(
            node.id,
            node.arguments.map { |n| function_replace(n, fn_id, replacement) },
            infix: node.infix
          )
        else
          node
        end
      end
      # rubocop:enable Metrics/MethodLength

      def resolve_function(fn_id)
        id = fn_id.to_sym
        return @functions[id] if defined_function?(id)

        ::CSVPlusPlus::Entities::Builtins::FUNCTIONS[id]
      end

      def call_function_or_builtin(function_or_builtin, function_call)
        if function_or_builtin.function?
          call_function(function_or_builtin, function_call)
        else
          function_or_builtin.resolve_fn.call(self, function_call.arguments)
        end
      end

      def call_function(function, function_call)
        i = 0
        function.arguments.reduce(function.body.dup) do |ast, argument|
          variable_replace(ast, argument, function_call.arguments[i]).tap do
            i += 1
          end
        end
      end

      # Make a copy of the AST represented by +node+ and replace +var_id+ with +replacement+ throughout
      def variable_replace(node, var_id, replacement)
        if node.function_call?
          arguments = node.arguments.map { |n| variable_replace(n, var_id, replacement) }
          # TODO: refactor these places where we copy functions... it's brittle with the kwargs
          ::CSVPlusPlus::Entities::FunctionCall.new(node.id, arguments, infix: node.infix)
        elsif node.variable? && node.id == var_id
          replacement
        else
          node
        end
      end

      def resolve_variable(var_id)
        id = var_id.to_sym
        return @variables[id] if defined_variable?(id)

        raise_formula_syntax_error('Undefined variable', var_id) unless builtin_variable?(var_id)

        ::CSVPlusPlus::Entities::Builtins::VARIABLES[var_id.to_sym].resolve_fn.call(self)
      end

      #       def check_unbound_vars(dependencies, variables)
      #         unbound_vars = dependencies.values.flatten - variables.keys
      #         return if unbound_vars.empty?
      #
      #         raise_formula_syntax_error('Undefined variables', unbound_vars.map(&:to_s).join(', '))
      #       end

      #       def variable_resolution_order(variables)
      #         # we have a hash of variables => ASTs but they might have references to each other, so
      #         # we need to interpolate them first (before interpolating the cell values)
      #         var_dependencies = ::CSVPlusPlus::Graph.dependency_graph(variables, @runtime)
      #         # are there any references that we don't have variables for? (undefined variable)
      #         check_unbound_vars(var_dependencies, variables)
      #
      #         # a topological sort will give us the order of dependencies
      #         [var_dependencies, ::CSVPlusPlus::Graph.topological_sort(var_dependencies)]
      #         # TODO: don't expose this exception directly to the caller
      #       rescue ::TSort::Cyclic
      #         @runtime.raise_formula_syntax_error('Cyclic variable dependency detected', var_refs.keys)
      #       end

      #       def resolve_dependencies(var_dependencies, resolution_order, variables)
      #         {}.tap do |resolved_vars|
      #           # for each var and each dependency it has, build up and mutate resolved_vars
      #           resolution_order.each do |var|
      #             resolved_vars[var] = variables[var].dup
      #
      #             var_dependencies[var].each do |dependency|
      #               resolved_vars[var] = variable_replace(resolved_vars[var], dependency, variables[dependency])
      #             end
      #           end
      #         end
      #       end
    end
  end
end

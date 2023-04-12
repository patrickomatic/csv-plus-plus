# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # Methods for resolving functions and variables.  These should be included onto a class that has +@variables+ and
    # +@functions+ instance variables.
    module CanResolveReferences
      extend ::T::Sig

      sig { void }
      # Resolve all values in the ast of the current cell being processed
      #
      # @return [Entity]
      def resolve_cell_value
        return unless (ast = @cell&.ast)

        last_round = nil
        ::Kernel.loop do
          refs = ::CSVPlusPlus::Runtime::References.extract(ast, self)
          return ast if refs.empty?

          # TODO: throw an error here instead I think - basically we did a round and didn't make progress
          return ast if last_round == refs

          ast = resolve_functions(resolve_variables(ast, refs.variables), refs.functions)
        end
      end

      sig { params(var_id: ::Symbol).returns(::T::Boolean) }
      # Variables outside of an ![[expand=...] are always in scope.  If it's defined within an expand then things
      # get trickier because the variable is only in scope while we're processing cells within that expand.
      #
      # @param var_id [Symbol] The variable's identifier that we are checking if it's in scope
      #
      # @return [boolean]
      def in_scope?(var_id)
        value = @variables[var_id]

        raise_modifier_syntax_error('Undefined variable reference', var_id.to_s) if value.nil?

        expand = value.type == ::CSVPlusPlus::Entities::Type::CellReference && value.scoped_to_expand
        return true unless expand

        unless expand.starts_at
          ::Kernel.raise(
            ::CSVPlusPlus::Error::Error,
            'Must call Template.expand_rows! before checking the scope of expands.'
          )
        end

        row_index >= expand.starts_at && (expand.ends_at.nil? || row_index <= expand.ends_at)
      end

      private

      sig do
        params(
          ast: ::CSVPlusPlus::Entities::Entity,
          refs: ::T::Array[::CSVPlusPlus::Entities::FunctionCall]
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
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
        if node.type == ::CSVPlusPlus::Entities::Type::FunctionCall && node.id == fn_id
          call_function_or_builtin(replacement, node)
        elsif node.type == ::CSVPlusPlus::Entities::Type::FunctionCall
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

      sig { params(fn_id: ::Symbol).returns(::T.any(::CSVPlusPlus::Runtime::Value, ::CSVPlusPlus::Entities::Function)) }
      def resolve_function(fn_id)
        return @functions[fn_id] if defined_function?(fn_id)

        builtin = ::CSVPlusPlus::Runtime::Builtins::FUNCTIONS[fn_id]
        raise_formula_syntax_error('Undefined function', fn_id.to_s) unless builtin

        builtin
      end

      sig do
        params(
          function_or_builtin: ::T.any(::CSVPlusPlus::Runtime::Value, ::CSVPlusPlus::Entities::Function),
          function_call: ::CSVPlusPlus::Entities::FunctionCall
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
      def call_function_or_builtin(function_or_builtin, function_call)
        if function_or_builtin.is_a?(::CSVPlusPlus::Runtime::Value)
          function_or_builtin.call(self, function_call.arguments)
        else
          call_function(function_or_builtin, function_call)
        end
      end

      sig do
        params(
          function: ::CSVPlusPlus::Entities::Function,
          function_call: ::CSVPlusPlus::Entities::FunctionCall
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
      def call_function(function, function_call)
        i = 0
        function.arguments.reduce(function.body.dup) do |ast, argument|
          variable_replace(ast, argument, function_call.arguments[i]).tap do
            i += 1
          end
        end
      end

      sig do
        params(
          node: ::CSVPlusPlus::Entities::Entity,
          var_id: ::Symbol,
          replacement: ::CSVPlusPlus::Entities::Entity
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
      # Make a copy of the AST represented by +node+ and replace +var_id+ with +replacement+ throughout
      def variable_replace(node, var_id, replacement)
        if node.is_a?(::CSVPlusPlus::Entities::FunctionCall)
          arguments = node.arguments.map { |n| variable_replace(n, var_id, replacement) }
          # TODO: refactor these places where we copy functions... it's brittle with the kwargs
          ::CSVPlusPlus::Entities::FunctionCall.new(node.id, arguments, infix: node.infix)
        elsif node.type == ::CSVPlusPlus::Entities::Type::Variable && node.id == var_id
          replacement
        else
          node
        end
      end

      sig { params(var_id: ::Symbol).returns(::T.any(::CSVPlusPlus::Runtime::Value, ::CSVPlusPlus::Entities::Entity)) }
      def resolve_variable(var_id)
        return @variables[id] if defined_variable?(id)

        raise_formula_syntax_error('Undefined variable', var_id) unless builtin_variable?(var_id)

        ::CSVPlusPlus::Runtime::Builtins::VARIABLES[var_id].evaluate(self)
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

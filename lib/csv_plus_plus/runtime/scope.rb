# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # Responsible for storing and resolving variables and function references
    # rubocop:disable Metrics/ClassLength
    class Scope
      extend ::T::Sig

      sig { returns(::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Function]) }
      attr_reader :functions

      sig { returns(::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity]) }
      attr_reader :variables

      sig do
        params(
          functions: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Function],
          variables: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity]
        ).void
      end
      # @param functions [Hash<Symbol, Function>] Pre-defined functions
      # @param variables [Hash<Symbol, Entity>] Pre-defined variables
      def initialize(functions: {}, variables: {})
        @functions = functions
        @variables = variables
      end

      sig { params(id: ::Symbol, entity: ::CSVPlusPlus::Entities::Entity).returns(::CSVPlusPlus::Entities::Entity) }
      # Define a (or re-define an existing) variable
      #
      # @param id [String, Symbol] The identifier for the variable
      # @param entity [Entity] The value (entity) the variable holds
      #
      # @return [Entity] The value of the variable (+entity+)
      def def_variable(id, entity)
        @variables[id] = entity
      end

      sig { params(vars: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity]).void }
      # Define (or re-define existing) variables
      #
      # @param vars [Hash<Symbol, Variable>] Variables to define
      def def_variables(vars)
        vars.each { |id, entity| def_variable(id, entity) }
      end

      sig do
        params(id: ::Symbol, function: ::CSVPlusPlus::Entities::Function).returns(::CSVPlusPlus::Entities::Function)
      end
      # Define a (or re-define an existing) function
      #
      # @param id [Symbol] The identifier for the function
      # @param function [Entities::Function] The defined function
      #
      # @return [Entities::Function] The defined function
      def def_function(id, function)
        @functions[id.to_sym] = function
      end

      sig { params(var_id: ::Symbol, position: ::CSVPlusPlus::Runtime::Position).returns(::T::Boolean) }
      # Variables outside of an ![[expand=...] are always in scope.  If it's defined within an expand then things
      # get trickier because the variable is only in scope while we're processing cells within that expand.
      #
      # @param var_id [Symbol] The variable's identifier that we are checking if it's in scope
      # @param position [Position]
      #
      # @return [boolean]
      def in_scope?(var_id, position)
        value = @variables[var_id]

        return false unless value

        expand = value.is_a?(::CSVPlusPlus::Entities::Reference) && value.a1_ref.scoped_to_expand
        !expand || expand.position_within?(position)
      end

      sig { returns(::String) }
      # Provide a summary of the functions and variables compiled (to show in verbose mode)
      #
      # @return [::String]
      def verbose_summary
        <<~SUMMARY
          # Code Section Summary

          ## Resolved Variables

          #{variable_summary}

          ## Functions

          #{function_summary}
        SUMMARY
      end

      sig do
        params(
          position: ::CSVPlusPlus::Runtime::Position,
          ast: ::CSVPlusPlus::Entities::Entity,
          refs: ::T::Enumerable[::CSVPlusPlus::Entities::FunctionCall]
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
      # @param position [Position
      # @param ast [Entity]
      # @param refs [Array<FunctionCall>]
      #
      # @return [Entity]
      def resolve_functions(position, ast, refs)
        refs.reduce(ast.dup) do |acc, elem|
          function_replace(position, acc, elem.id, resolve_function(elem.id))
        end
      end

      sig do
        params(
          position: ::CSVPlusPlus::Runtime::Position,
          ast: ::CSVPlusPlus::Entities::Entity,
          refs: ::T::Enumerable[::CSVPlusPlus::Entities::Reference]
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
      # @param position [Position]
      # @param ast [Entity]
      # @param refs [Array<Variable>]
      #
      # @return [Entity]
      def resolve_variables(position, ast, refs)
        refs.reduce(ast.dup) do |acc, elem|
          next acc unless (id = elem.id)

          variable_replace(acc, id, resolve_variable(position, id))
        end
      end

      sig do
        params(
          position: ::CSVPlusPlus::Runtime::Position,
          node: ::CSVPlusPlus::Entities::Entity,
          fn_id: ::Symbol,
          replacement: ::T.any(::CSVPlusPlus::Entities::Function, ::CSVPlusPlus::Entities::RuntimeValue)
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
      # Make a copy of the AST represented by +node+ and replace +fn_id+ with +replacement+ throughout
      # rubocop:disable Metrics/MethodLength
      def function_replace(position, node, fn_id, replacement)
        if node.is_a?(::CSVPlusPlus::Entities::FunctionCall) && node.id == fn_id
          call_function_or_builtin(position, replacement, node)
        elsif node.is_a?(::CSVPlusPlus::Entities::FunctionCall)
          # not our function, but continue our depth first search on it
          ::CSVPlusPlus::Entities::FunctionCall.new(
            node.id,
            node.arguments.map { |n| function_replace(position, n, fn_id, replacement) },
            infix: node.infix
          )
        else
          node
        end
      end
      # rubocop:enable Metrics/MethodLength

      sig do
        params(fn_id: ::Symbol)
          .returns(::T.any(::CSVPlusPlus::Entities::Function, ::CSVPlusPlus::Entities::RuntimeValue))
      end
      # @param fn_id [Symbol]
      #
      # @return [Entities::Function]
      def resolve_function(fn_id)
        return ::T.must(@functions[fn_id]) if @functions.key?(fn_id)

        builtin = ::CSVPlusPlus::Entities::Builtins::FUNCTIONS[fn_id]
        raise(::CSVPlusPlus::Error::FormulaSyntaxError.new('Undefined function', bad_input: fn_id.to_s)) unless builtin

        builtin
      end

      sig do
        params(
          position: ::CSVPlusPlus::Runtime::Position,
          function_or_builtin: ::T.any(::CSVPlusPlus::Entities::RuntimeValue, ::CSVPlusPlus::Entities::Function),
          function_call: ::CSVPlusPlus::Entities::FunctionCall
        ).returns(::CSVPlusPlus::Entities::Entity)
      end
      # @param position [Position]
      # @param function_or_builtin [Entities::Function, Entities::RuntimeValue]
      # @param function_call [Entities::FunctionCall]
      #
      # @return [Entities::Entity]
      def call_function_or_builtin(position, function_or_builtin, function_call)
        if function_or_builtin.is_a?(::CSVPlusPlus::Entities::RuntimeValue)
          function_or_builtin.call(position, function_call.arguments)
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
      # Since functions are just built up from existing functions, a "call" is effectively replacing the variable
      # references in the +@body+ with the ones being passed as arguments
      #
      # @param function [Entities::Function] The function being called
      # @param function_call [Entities::FunctionCall] The actual call of the function
      #
      # @return [Entities::Entity]
      def call_function(function, function_call)
        i = 0
        function.arguments.reduce(function.body.dup) do |ast, argument|
          variable_replace(ast, argument, ::T.must(function_call.arguments[i])).tap do
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
        elsif node.is_a?(::CSVPlusPlus::Entities::Reference) && node.id == var_id
          replacement
        else
          node
        end
      end

      sig do
        params(position: ::CSVPlusPlus::Runtime::Position, var_id: ::Symbol).returns(::CSVPlusPlus::Entities::Entity)
      end
      # @param position [Position]
      # @param var_id [Symbol]
      #
      # @return [Entities::Entity]
      def resolve_variable(position, var_id)
        return ::T.must(@variables[var_id]) if @variables.key?(var_id)

        unless ::CSVPlusPlus::Entities::Builtins.builtin_variable?(var_id)
          raise(::CSVPlusPlus::Error::FormulaSyntaxError.new('Undefined variable', bad_input: var_id.to_s))
        end

        ::T.must(::CSVPlusPlus::Entities::Builtins::VARIABLES[var_id]).call(position, [])
      end

      sig { returns(::String) }
      # Create a summary of all currently defined variables
      #
      # @return [String]
      def variable_summary
        return '(no variables defined)' if @variables.empty?

        @variables.map { |k, v| "#{k} := #{v}" }
                  .join("\n")
      end

      sig { returns(::String) }
      # Create a summary of all currently defined functions
      #
      # @return [String]
      def function_summary
        return '(no functions defined)' if @functions.empty?

        @functions.map { |k, f| "#{k}: #{f}" }
                  .join("\n")
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end

# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # Methods for classes that need to manage +@variables+ and +@functions+
    module CanDefineReferences
      extend ::T::Sig

      sig { params(var_id: ::Symbol).returns(::CSVPlusPlus::Entities::CellReference) }
      # Bind +var_id+ to the current cell
      #
      # @param var_id [Symbol] The name of the variable to bind the cell reference to
      #
      # @return [CellReference]
      def bind_variable_to_cell(var_id)
        def_variable(var_id, ::CSVPlusPlus::Entities::CellReference.new(cell_index:, row_index:))
      end

      sig do
        params(
          var_id: ::Symbol,
          expand: ::CSVPlusPlus::Modifier::Expand
        ).returns(::CSVPlusPlus::Entities::CellReference)
      end
      # Bind +var_id+ relative to an ![[expand]] modifier.
      #
      # @param var_id [Symbol] The name of the variable to bind the cell reference to
      # @param expand [Expand] The expand where the variable is accessible (where it will be bound relative to)
      #
      # @return [CellReference]
      def bind_variable_in_expand(var_id, expand)
        def_variable(var_id, ::CSVPlusPlus::Entities::CellReference.new(scoped_to_expand: expand, cell_index:))
      end

      sig { params(id: ::Symbol, entity: ::CSVPlusPlus::Entities::Entity).returns(::CSVPlusPlus::Entities::Entity) }
      # Define a (or re-define an existing) variable
      #
      # @param id [String, Symbol] The identifier for the variable
      # @param entity [Entity] The value (entity) the variable holds
      #
      # @return [Entity] The value of the variable (+entity+)
      def def_variable(id, entity)
        @variables[id.to_sym] = entity
      end

      sig { params(vars: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Variable]).void }
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

      sig { params(var_id: ::Symbol).returns(::T::Boolean) }
      # Is the variable defined?
      #
      # @param var_id [Symbol, ::String] The identifier of the variable
      #
      # @return [T::Boolean]
      def defined_variable?(var_id)
        @variables.key?(var_id.to_sym)
      end

      sig { params(fn_id: ::Symbol).returns(::T::Boolean) }
      # Is the function defined?
      #
      # @param fn_id [Symbol, String] The identifier of the function
      #
      # @return [T::Boolean]
      def defined_function?(fn_id)
        @functions.key?(fn_id.to_sym)
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

      private

      sig { returns(::String) }
      def variable_summary
        return '(no variables defined)' if @variables.empty?

        @variables.map { |k, v| "#{k} := #{v}" }
                  .join("\n")
      end

      sig { returns(::String) }
      def function_summary
        return '(no functions defined)' if @functions.empty?

        @functions.map { |k, f| "#{k}: #{f}" }
                  .join("\n")
      end
    end
  end
end

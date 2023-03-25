# frozen_string_literal: true

module CSVPlusPlus
  # Methods for classes that need to manage +@variables+ and +@functions+
  module CanDefineReferences
    # Define a (or re-define an existing) variable
    #
    # @param id [String, Symbol] The identifier for the variable
    # @param entity [Entity] The value (entity) the variable holds
    #
    # @return [Entity] The value of the variable (+entity+)
    def def_variable(id, entity)
      variables[id.to_sym] = entity
    end

    # Define (or re-define existing) variables
    #
    # @param vars [Hash<Symbol, Variable>] Variables to define
    def def_variables(vars)
      vars.each { |id, entity| def_variable(id, entity) }
    end

    # Define a (or re-define an existing) function
    #
    # @param id [String, Symbol] The identifier for the function
    # @param entity [Entities::Function] The defined function
    #
    # @return [Entities::Function] The defined function
    def def_function(id, entity)
      functions[id.to_sym] = entity
    end

    # Is the variable defined?
    #
    # @param var_id [Symbol, String] The identifier of the variable
    #
    # @return [boolean]
    def defined_variable?(var_id)
      variables.key?(var_id.to_sym)
    end

    # Is the function defined?
    #
    # @param fn_id [Symbol, String] The identifier of the function
    #
    # @return [boolean]
    def defined_function?(fn_id)
      functions.key?(fn_id.to_sym)
    end

    # Provide a summary of the functions and variables compiled (to show in verbose mode)
    #
    # @return [String]
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

    def variables
      @variables ||= {}
    end

    def functions
      @functions ||= {}
    end

    def variable_summary
      return '(no variables defined)' if variables.empty?

      variables.map { |k, v| "#{k} := #{v}" }
               .join("\n")
    end

    def function_summary
      return '(no functions defined)' if functions.empty?

      functions.map { |k, f| "#{k}: #{f}" }
               .join("\n")
    end
  end
end

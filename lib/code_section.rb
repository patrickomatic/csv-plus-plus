# frozen_string_literal: true

require_relative './language/code_section.tab'
require_relative './language/entities'

module CSVPlusPlus
  ##
  # A representation of the code section part of a template (the variable and function definitions)
  class CodeSection
    attr_reader :functions, :variables

    # initialize
    def initialize(variables: {}, functions: {})
      @variables = variables
      @functions = functions
    end

    # Define a (or re-define an existing) variable
    def def_variable(id, entity)
      @variables[id.to_sym] = entity
    end

    # Define (or re-define existing) variables
    def def_variables(variables)
      variables.each { |id, entity| def_variable(id, entity) }
    end

    # Define a (or re-define an existing) function
    def def_function(id, entity)
      @functions[id.to_sym] = entity
    end

    # Is the variable defined?
    def defined_variable?(var_id)
      @variables.key?(var_id.to_sym)
    end

    # Is the function defined?
    def defined_function?(fn_id)
      @functions.key?(fn_id.to_sym)
    end

    # to_s
    def to_s
      "CodeSection(functions: #{@functions}, variables: #{@variables})"
    end
  end
end

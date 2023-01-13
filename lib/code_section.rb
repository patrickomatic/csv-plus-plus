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
    def def_function(id, arguments, body)
      @function[id.to_sym] = ::CSVPlusPlus::Language::Function.new(id, arguments, body)
    end

    # to_s
    def to_s
      "CodeSection(variables: #{@variables} functions: #{@functions})"
    end
  end
end

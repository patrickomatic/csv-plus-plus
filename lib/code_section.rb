# frozen_string_literal: true

require_relative './language/code_section.tab'
require_relative './language/entities'
require_relative './language/syntax_error'

module CSVPlusPlus
  ##
  # A representation of the code section part of a template (the variable and function definitions)
  class CodeSection
    attr_reader :functions, :variables

    # Parse a file into an instance of CodeSection
    def self.parse(execution_context, key_values = {})
      ::CSVPlusPlus::Language::CodeSectionParser.new.parse(execution_context).tap do |c|
        # TODO: infer a type
        # allow user-supplied key/values to override anything global or from the code section
        c.def_variables(key_values.transform_values { |v| ::CSVPlusPlus::Language::String.new(v.to_s) })

        resolved_variables = execution_context.resolve_static_variables!(c)
        # statically resolve all non-runtime variables
        c.def_variables(resolved_variables)
      end
    end

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

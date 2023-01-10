require_relative './language/code_section.tab'
require_relative './language/entities'
require_relative './language/syntax_error'

module CSVPlusPlus
  class CodeSection
    attr_reader :functions, :variables

    def initialize(variables: {}, functions: {})
      @variables = variables
      @functions = functions
    end

    def self.parse(execution_context, key_values = {})
      Language::CodeSectionParser.new.parse(execution_context).tap do |c|
        # TODO infer a type
        # allow user-supplied key/values to override anything global or from the code section
        c.def_variables(Hash[key_values.map {|k, v| [k, Language::String.new(v.to_s)]}])
        
        resolved_variables = execution_context.resolve_static_variables!(c)
        # statically resolve all non-runtime variables
        c.def_variables(resolved_variables)
      end
    end

    def def_variable(id, entity)
      @variables[id.to_sym] = entity
    end

    def def_variables(variables)
      variables.each {|id, entity| def_variable(id, entity)}
    end

    def def_function(id, arguments, body)
      @function[id.to_sym] = Language::Function.new(id, arguments, body)
    end

    def to_s
      "CodeSection(variables: #{@variables.to_s} functions: #{@functions.to_s})"
    end
  end
end

# frozen_string_literal: true

require_relative 'entities/boolean'
require_relative 'entities/cell_reference'
require_relative 'entities/entity'
require_relative 'entities/function'
require_relative 'entities/function_call'
require_relative 'entities/number'
require_relative 'entities/runtime_value'
require_relative 'entities/string'
require_relative 'entities/variable'

TYPES = %i[function function_call cell_reference boolean number string variable runtime_value].freeze

module CSVPlusPlus
  module Language
    # TODO: move this into a lexer class eventually
    END_OF_CODE_SECTION = '---'
    public_constant :END_OF_CODE_SECTION

    VARIABLE_REF = '$$'
    public_constant :VARIABLE_REF
  end
end

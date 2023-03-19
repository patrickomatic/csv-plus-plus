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

module CSVPlusPlus
  module Entities
    TYPES = {
      boolean: ::CSVPlusPlus::Entities::Boolean,
      cell_reference: ::CSVPlusPlus::Entities::CellReference,
      function: ::CSVPlusPlus::Entities::Function,
      function_call: ::CSVPlusPlus::Entities::FunctionCall,
      number: ::CSVPlusPlus::Entities::Number,
      runtime_value: ::CSVPlusPlus::Entities::RuntimeValue,
      string: ::CSVPlusPlus::Entities::String,
      variable: ::CSVPlusPlus::Entities::Variable
    }.freeze

    public_constant :TYPES
  end
end

require_relative 'entities/ast_builder'
require_relative 'entities/builtins'

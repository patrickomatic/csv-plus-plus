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
  module Language
    TYPES = {
      boolean: ::CSVPlusPlus::Language::Entities::Boolean,
      cell_reference: ::CSVPlusPlus::Language::Entities::CellReference,
      function: ::CSVPlusPlus::Language::Entities::Function,
      function_call: ::CSVPlusPlus::Language::Entities::FunctionCall,
      number: ::CSVPlusPlus::Language::Entities::Number,
      runtime_value: ::CSVPlusPlus::Language::Entities::RuntimeValue,
      string: ::CSVPlusPlus::Language::Entities::String,
      variable: ::CSVPlusPlus::Language::Entities::Variable
    }.freeze

    public_constant :TYPES
  end
end

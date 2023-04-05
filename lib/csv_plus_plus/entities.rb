# typed: strict
# frozen_string_literal: true

require_relative 'entities/entity'
require_relative 'entities/entity_with_arguments'

require_relative 'entities/boolean'
require_relative 'entities/cell_reference'
require_relative 'entities/date'
require_relative 'entities/function'
require_relative 'entities/function_call'
require_relative 'entities/number'
require_relative 'entities/runtime_value'
require_relative 'entities/string'
require_relative 'entities/variable'

module CSVPlusPlus
  # The entities that form abstract syntax trees which make up the language
  module Entities
    extend ::T::Sig

    # A primitive type.  These all correspond to an implementation of the same name in the CSVPlusPlus::Entities module.
    class Type < ::T::Enum
      enums do
        Boolean = new
        CellReference = new
        Date = new
        Function = new
        FunctionCall = new
        Number = new
        RuntimeValue = new
        String = new
        Variable = new
      end
    end
  end
end

require_relative 'entities/ast_builder'
require_relative 'entities/builtins'

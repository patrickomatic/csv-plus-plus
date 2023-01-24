# frozen_string_literal: true

require_relative './lexer/lexer'
require_relative './lexer/tokenizer'

module CSVPlusPlus
  module Lexer
    END_OF_CODE_SECTION = '---'
    public_constant :END_OF_CODE_SECTION

    VARIABLE_REF = '$$'
    public_constant :VARIABLE_REF
  end
end

#
# DO NOT MODIFY!!!!
# This file is automatically generated by Racc 1.6.2
# from Racc grammar file "".
#

require 'racc/parser.rb'

  require_relative '../lexer'
  require_relative '../entities/ast_builder'

module CSVPlusPlus
  module Parser
    class CellValue < Racc::Parser

module_eval(<<'...end cell_value.y/module_eval...', 'cell_value.y', 49)
  include ::CSVPlusPlus::Entities::ASTBuilder
  include ::CSVPlusPlus::Lexer

  protected

  def anything_to_parse?(input)
    input.strip.start_with?('=')
  end

  def parse_subject
    'cell value'
  end

  def return_value
    @ast
  end

  def tokenizer
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /[\{\}\(\),=]/,
      ignore: /\s+/,
      tokens: [
        TOKEN_LIBRARY[:TRUE],
        TOKEN_LIBRARY[:FALSE],
        TOKEN_LIBRARY[:NUMBER],
        TOKEN_LIBRARY[:STRING],
        TOKEN_LIBRARY[:INFIX_OP],
        TOKEN_LIBRARY[:VAR_REF],
        TOKEN_LIBRARY[:ID]
      ]
    )
  end
...end cell_value.y/module_eval...
##### State transition tables begin ###

racc_action_table = [
     7,    21,    15,    25,     2,    16,     3,     7,    14,    18,
    19,    16,    16,    16,     7,    12,    13,    16,    10,     9,
    11,     8,    12,    13,    26,    10,     9,    11,     8,    12,
    13,   nil,    10,     9,    11,     8,     7,    23,   nil,   nil,
   nil,   nil,   nil,     7,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,    12,    13,   nil,    10,     9,    11,     8,    12,    13,
   nil,    10,     9,    11,     8 ]

racc_action_check = [
     2,    17,     4,    22,     0,     4,     1,     7,     3,     8,
    13,    20,    24,    27,    16,     2,     2,    17,     2,     2,
     2,     2,     7,     7,    22,     7,     7,     7,     7,    16,
    16,   nil,    16,    16,    16,    16,    19,    19,   nil,   nil,
   nil,   nil,   nil,    26,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,    19,    19,   nil,    19,    19,    19,    19,    26,    26,
   nil,    26,    26,    26,    26 ]

racc_action_pointer = [
    -6,     6,    -2,     8,   -14,   nil,   nil,     5,    -9,   nil,
   nil,   nil,   nil,     8,   nil,   nil,    12,    -2,   nil,    34,
    -8,   nil,     0,   nil,    -7,   nil,    41,    -6 ]

racc_action_default = [
   -16,   -16,   -16,   -16,   -16,    -2,    -3,   -16,   -16,    -6,
    -7,    -8,    -9,   -10,    28,    -1,   -16,   -16,    -5,   -16,
   -15,    -4,   -16,   -12,   -14,   -11,   -16,   -13 ]

racc_goto_table = [
     4,     1,    22,   nil,   nil,    17,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,    20,   nil,   nil,    24,   nil,   nil,
   nil,   nil,   nil,   nil,    27 ]

racc_goto_check = [
     2,     1,     5,   nil,   nil,     2,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,     2,   nil,   nil,     2,   nil,   nil,
   nil,   nil,   nil,   nil,     2 ]

racc_goto_pointer = [
   nil,     1,    -2,   nil,   nil,   -17 ]

racc_goto_default = [
   nil,   nil,   nil,     5,     6,   nil ]

racc_reduce_table = [
  0, 0, :racc_error,
  3, 26, :_reduce_1,
  1, 27, :_reduce_none,
  1, 27, :_reduce_none,
  3, 27, :_reduce_4,
  2, 27, :_reduce_5,
  1, 27, :_reduce_6,
  1, 27, :_reduce_7,
  1, 27, :_reduce_8,
  1, 27, :_reduce_9,
  1, 27, :_reduce_10,
  4, 28, :_reduce_11,
  3, 28, :_reduce_12,
  3, 30, :_reduce_13,
  1, 30, :_reduce_14,
  3, 29, :_reduce_15 ]

racc_reduce_n = 16

racc_shift_n = 28

racc_token_table = {
  false => 0,
  :error => 1,
  "(" => 2,
  ")" => 3,
  "^" => 4,
  "*" => 5,
  "/" => 6,
  "+" => 7,
  "-" => 8,
  "&" => 9,
  "=" => 10,
  "<" => 11,
  ">" => 12,
  "<=" => 13,
  ">=" => 14,
  "<>" => 15,
  :EOL => 16,
  :FALSE => 17,
  :ID => 18,
  :INFIX_OP => 19,
  :NUMBER => 20,
  :STRING => 21,
  :TRUE => 22,
  :VAR_REF => 23,
  "," => 24 }

racc_nt_base = 25

racc_use_result_var = true

Racc_arg = [
  racc_action_table,
  racc_action_check,
  racc_action_default,
  racc_action_pointer,
  racc_goto_table,
  racc_goto_check,
  racc_goto_default,
  racc_goto_pointer,
  racc_nt_base,
  racc_reduce_table,
  racc_token_table,
  racc_shift_n,
  racc_reduce_n,
  racc_use_result_var ]

Racc_token_to_s_table = [
  "$end",
  "error",
  "\"(\"",
  "\")\"",
  "\"^\"",
  "\"*\"",
  "\"/\"",
  "\"+\"",
  "\"-\"",
  "\"&\"",
  "\"=\"",
  "\"<\"",
  "\">\"",
  "\"<=\"",
  "\">=\"",
  "\"<>\"",
  "EOL",
  "FALSE",
  "ID",
  "INFIX_OP",
  "NUMBER",
  "STRING",
  "TRUE",
  "VAR_REF",
  "\",\"",
  "$start",
  "cell_value",
  "exp",
  "fn_call",
  "infix_fn_call",
  "fn_call_args" ]

Racc_debug_parser = false

##### State transition tables end #####

# reduce 0 omitted

module_eval(<<'.,.,', 'cell_value.y', 21)
  def _reduce_1(val, _values, result)
     @ast = val[1]
    result
  end
.,.,

# reduce 2 omitted

# reduce 3 omitted

module_eval(<<'.,.,', 'cell_value.y', 25)
  def _reduce_4(val, _values, result)
     result = val[1]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 26)
  def _reduce_5(val, _values, result)
     result = variable(val[1])
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 27)
  def _reduce_6(val, _values, result)
     result = string(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 28)
  def _reduce_7(val, _values, result)
     result = number(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 29)
  def _reduce_8(val, _values, result)
     result = boolean(true)
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 30)
  def _reduce_9(val, _values, result)
     result = boolean(false)
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 31)
  def _reduce_10(val, _values, result)
     result = cell_reference(ref: val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 33)
  def _reduce_11(val, _values, result)
     result = function_call(val[0], val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 34)
  def _reduce_12(val, _values, result)
     result = function_call(val[0], [])
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 36)
  def _reduce_13(val, _values, result)
     result = val[0] << val[2]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 37)
  def _reduce_14(val, _values, result)
     result = [val[0]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 39)
  def _reduce_15(val, _values, result)
     result = function_call(val[1], [val[0], val[2]], infix: true)
    result
  end
.,.,

def _reduce_none(val, _values, result)
  val[0]
end

    end   # class CellValue
  end   # module Parser
end   # module CSVPlusPlus

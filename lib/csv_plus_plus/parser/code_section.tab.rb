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
    class CodeSection < Racc::Parser

module_eval(<<'...end code_section.y/module_eval...', 'code_section.y', 69)
  include ::CSVPlusPlus::Lexer
  include ::CSVPlusPlus::Entities::ASTBuilder

  protected

  def anything_to_parse?(input)
    @rest = input.strip

    return !@rest.index(::CSVPlusPlus::Lexer::END_OF_CODE_SECTION).nil?
  end

  def parse_subject
    'code section'
  end

  def tokenizer
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /[\{\}\(\),]/, # TODO: do I even need this (oh I think brackets are for arrays
      ignore: /\s+|\#[^\n]+\n/,
      stop_fn: lambda do |scanner|
        return false unless scanner.scan(/#{::CSVPlusPlus::Lexer::END_OF_CODE_SECTION}/)

        @tokens << [:END_OF_CODE, scanner.matched]
        @rest = scanner.rest.strip
        true
      end,
      tokens: [
        [/:=/, :ASSIGN],
        [/def/, :FN_DEF],
        TOKEN_LIBRARY[:TRUE],
        TOKEN_LIBRARY[:FALSE],
        TOKEN_LIBRARY[:NUMBER],
        TOKEN_LIBRARY[:STRING],
        TOKEN_LIBRARY[:INFIX_OP],
        TOKEN_LIBRARY[:VAR_REF],
        TOKEN_LIBRARY[:ID]
      ],
    )
  end

  def return_value
    @rest
  end

  private

  def def_function(id, arguments, body)
    fn_def = function(id.to_sym, arguments, body)
    @runtime.def_function(fn_def.id, fn_def)
  end

  def def_variable(id, ast)
    @runtime.def_variable(id.to_sym, ast)
  end
...end code_section.y/module_eval...
##### State transition tables begin ###

racc_action_table = [
    20,    38,     9,    12,    13,    14,    16,    20,    31,    33,
    34,    31,    42,    31,    20,    31,    29,    25,    26,    31,
    23,    22,    24,    21,    25,    26,    20,    23,    22,    24,
    21,    25,    26,    30,    23,    22,    24,    21,    20,    41,
    31,   nil,    35,    25,    26,    20,    23,    22,    24,    21,
     3,    10,   nil,     7,     7,    25,    26,    36,    23,    22,
    24,    21,    25,    26,    43,    23,    22,    24,    21,     8,
     8,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,    44 ]

racc_action_check = [
    13,    32,     1,     7,     8,     9,    12,    15,    17,    21,
    26,    27,    36,    37,    20,    39,    16,    13,    13,    32,
    13,    13,    13,    13,    15,    15,    31,    15,    15,    15,
    15,    20,    20,    16,    20,    20,    20,    20,    34,    34,
    45,   nil,    28,    31,    31,    44,    31,    31,    31,    31,
     0,     2,   nil,     0,     2,    34,    34,    28,    34,    34,
    34,    34,    44,    44,    40,    44,    44,    44,    44,     0,
     2,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,    40 ]

racc_action_pointer = [
    48,     2,    49,   nil,   nil,   nil,   nil,   -18,    -2,     5,
   nil,   nil,     3,    -3,   nil,     4,    12,   -14,   nil,   nil,
    11,   -12,   nil,   nil,   nil,   nil,     7,   -11,    38,   nil,
   nil,    23,    -3,   nil,    35,   nil,    -9,    -9,   nil,    -7,
    60,   nil,   nil,   nil,    42,    18 ]

racc_action_default = [
   -27,   -27,   -27,    -2,    -4,    -5,    -6,   -27,   -27,   -27,
    -1,    -3,   -27,   -27,    46,   -27,   -27,   -12,   -13,   -14,
   -27,   -27,   -17,   -18,   -19,   -20,   -21,    -7,   -27,    -9,
   -11,   -27,   -27,   -16,   -27,    -8,   -27,   -22,   -15,   -26,
   -27,   -24,   -10,   -23,   -27,   -25 ]

racc_goto_table = [
    17,     4,    27,    11,     1,     2,    15,    32,    28,    40,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,    37,   nil,
   nil,    39,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,    45 ]

racc_goto_check = [
     7,     3,     7,     3,     1,     2,     6,     7,     8,    11,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,     7,   nil,
   nil,     7,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,     7 ]

racc_goto_pointer = [
   nil,     4,     5,     1,   nil,   nil,    -6,   -13,    -8,   nil,
   nil,   -25 ]

racc_goto_default = [
   nil,   nil,   nil,   nil,     5,     6,   nil,   nil,   nil,    18,
    19,   nil ]

racc_reduce_table = [
  0, 0, :racc_error,
  2, 28, :_reduce_none,
  1, 28, :_reduce_none,
  2, 29, :_reduce_none,
  1, 29, :_reduce_none,
  1, 30, :_reduce_none,
  1, 30, :_reduce_none,
  4, 31, :_reduce_7,
  3, 33, :_reduce_8,
  2, 33, :_reduce_9,
  3, 35, :_reduce_10,
  1, 35, :_reduce_11,
  3, 32, :_reduce_12,
  1, 34, :_reduce_none,
  1, 34, :_reduce_none,
  3, 34, :_reduce_15,
  2, 34, :_reduce_16,
  1, 34, :_reduce_17,
  1, 34, :_reduce_18,
  1, 34, :_reduce_19,
  1, 34, :_reduce_20,
  1, 34, :_reduce_21,
  3, 37, :_reduce_22,
  4, 36, :_reduce_23,
  3, 36, :_reduce_24,
  3, 38, :_reduce_25,
  1, 38, :_reduce_26 ]

racc_reduce_n = 27

racc_shift_n = 46

racc_token_table = {
  false => 0,
  :error => 1,
  :END_OF_CODE => 2,
  "(" => 3,
  ")" => 4,
  :FN_DEF => 5,
  :ASSIGN => 6,
  "^" => 7,
  "*" => 8,
  "/" => 9,
  "+" => 10,
  "-" => 11,
  "&" => 12,
  "=" => 13,
  "<" => 14,
  ">" => 15,
  "<=" => 16,
  ">=" => 17,
  "<>" => 18,
  "," => 19,
  :FALSE => 20,
  :ID => 21,
  :INFIX_OP => 22,
  :NUMBER => 23,
  :STRING => 24,
  :TRUE => 25,
  :VAR_REF => 26 }

racc_nt_base = 27

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
  "END_OF_CODE",
  "\"(\"",
  "\")\"",
  "FN_DEF",
  "ASSIGN",
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
  "\",\"",
  "FALSE",
  "ID",
  "INFIX_OP",
  "NUMBER",
  "STRING",
  "TRUE",
  "VAR_REF",
  "$start",
  "code_section",
  "code",
  "def",
  "fn_def",
  "var_def",
  "fn_def_args_or_not",
  "exp",
  "fn_def_args",
  "fn_call",
  "infix_fn_call",
  "fn_call_args" ]

Racc_debug_parser = false

##### State transition tables end #####

# reduce 0 omitted

# reduce 1 omitted

# reduce 2 omitted

# reduce 3 omitted

# reduce 4 omitted

# reduce 5 omitted

# reduce 6 omitted

module_eval(<<'.,.,', 'code_section.y', 33)
  def _reduce_7(val, _values, result)
     def_function(val[1], val[2], val[3])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 35)
  def _reduce_8(val, _values, result)
     result = val[1]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 36)
  def _reduce_9(val, _values, result)
     result = []
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 38)
  def _reduce_10(val, _values, result)
     result = val[0] << val[2]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 39)
  def _reduce_11(val, _values, result)
     result = [val[0]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 41)
  def _reduce_12(val, _values, result)
     def_variable(val[0], val[2])
    result
  end
.,.,

# reduce 13 omitted

# reduce 14 omitted

module_eval(<<'.,.,', 'code_section.y', 45)
  def _reduce_15(val, _values, result)
     result = val[1]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 46)
  def _reduce_16(val, _values, result)
     result = variable(val[1].to_sym)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 47)
  def _reduce_17(val, _values, result)
     result = string(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 48)
  def _reduce_18(val, _values, result)
     result = number(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 49)
  def _reduce_19(val, _values, result)
     result = boolean(true)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 50)
  def _reduce_20(val, _values, result)
     result = boolean(false)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 51)
  def _reduce_21(val, _values, result)
     result = cell_reference(ref: val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 53)
  def _reduce_22(val, _values, result)
     result = function_call(val[1].to_sym, [val[0], val[2]], infix: true)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 55)
  def _reduce_23(val, _values, result)
     result = function_call(val[0].to_sym, val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 56)
  def _reduce_24(val, _values, result)
     result = function_call(val[0].to_sym, [])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 58)
  def _reduce_25(val, _values, result)
     result = val[0] << val[2]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 59)
  def _reduce_26(val, _values, result)
     result = [val[0]]
    result
  end
.,.,

def _reduce_none(val, _values, result)
  val[0]
end

    end   # class CodeSection
  end   # module Parser
end   # module CSVPlusPlus

#
# DO NOT MODIFY!!!!
# This file is automatically generated by Racc 1.6.2
# from Racc grammar file "".
#

require 'racc/parser.rb'

require 'strscan'
require_relative '../code_section'
require_relative 'entities'

module CSVPlusPlus
  module Language
    class CodeSectionParser < Racc::Parser

module_eval(<<'...end code_section.y/module_eval...', 'code_section.y', 58)
  def def_function(id, arguments, body)
    fn_call = ::CSVPlusPlus::Language::Function.new(id, arguments, body)
    @code_section.def_function(fn_call.id, fn_call)
  end

  def def_variable(id, ast)
    @code_section.def_variable(id, ast)
  end

  def parse(input, runtime)
    text = input.read.strip
    @code_section = CodeSection.new

    eoc = ::CSVPlusPlus::Language::END_OF_CODE_SECTION
    eoc_index = text.index(eoc)
    return @code_section, text if eoc_index.nil?

    tokens, rest = [], ''

    s = StringScanner.new(text)
    until s.empty?
      case
      when s.scan(/\s+/)
      when s.scan(/\#[^\n]+\n/)
      when s.scan(/#{eoc}/)
        tokens << [:END_OF_CODE, s.matched]
        rest = s.rest.strip
        break
      when s.scan(/\n/)
        tokens << [:EOL, s.matched]
      when s.scan(/:=/)
        tokens << [:ASSIGN, s.matched]
      when s.scan(/\bdef\b/)
        tokens << [:FN_DEF, s.matched]
      when s.scan(/TRUE/)
        tokens << [:TRUE, s.matched]
      when s.scan(/FALSE/)
        tokens << [:FALSE, s.matched]
      when s.scan(/"(?:[^"\\]|\\(?:["\\\/bfnrt]|u[0-9a-fA-F]{4}))*"/)
        tokens << [:STRING, s.matched]
      when s.scan(/-?[\d.]+/)
        tokens << [:NUMBER, s.matched]
      when s.scan(/\$\$/)
        tokens << [:VAR_REF, s.matched]
      when s.scan(/[\w_]+/)
        tokens << [:ID, s.matched]
      when s.scan(/[\(\)\{\}\/\*\+\-,=&]/)
        tokens << [s.matched, s.matched]
      else
        runtime.raise_syntax_error('Unable to parse code section starting at', s.peek(100))
      end
    end

    return @code_section, rest if tokens.empty?

    define_singleton_method(:next_token) { tokens.shift }

    begin
      do_parse
    rescue Racc::ParseError => e
      runtime.raise_syntax_error('Error parsing code section', e.message, wrapped_error: e)
    end

    return @code_section, rest
  end
...end code_section.y/module_eval...
##### State transition tables begin ###

racc_action_table = [
    32,    28,    29,     9,    22,    17,    20,    19,    21,    18,
     7,     7,    24,    36,    37,     3,    10,    25,     8,     8,
    22,    17,    20,    19,    21,    18,    22,    17,    20,    19,
    21,    18,    22,    17,    20,    19,    21,    18,    22,    17,
    20,    19,    21,    18,    12,    13,    14,    15,    26,    27,
    35,    38 ]

racc_action_check = [
    26,    23,    23,     1,    26,    26,    26,    26,    26,    26,
     0,     2,    15,    31,    31,     0,     2,    15,     0,     2,
    13,    13,    13,    13,    13,    13,    24,    24,    24,    24,
    24,    24,    28,    28,    28,    28,    28,    28,    37,    37,
    37,    37,    37,    37,     7,     8,     9,    12,    17,    18,
    29,    33 ]

racc_action_pointer = [
     8,     3,     9,   nil,   nil,   nil,   nil,    34,    42,    46,
   nil,   nil,    43,    11,   nil,     7,   nil,    44,    39,   nil,
   nil,   nil,   nil,    -4,    17,   nil,    -5,   nil,    23,    40,
   nil,     8,   nil,    46,   nil,   nil,   nil,    29,   nil,   nil ]

racc_action_default = [
   -23,   -23,   -23,    -2,    -4,    -5,    -6,   -23,   -23,   -23,
    -1,    -3,   -23,   -23,    40,   -23,   -11,   -20,   -23,   -16,
   -17,   -18,   -19,   -23,   -23,   -10,   -23,   -15,   -23,   -23,
    -8,   -23,   -13,   -22,    -7,    -9,   -12,   -23,   -14,   -21 ]

racc_goto_table = [
    16,     4,     1,    11,     2,    23,    31,   nil,   nil,   nil,
   nil,    30,   nil,    33,   nil,    34,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,    39 ]

racc_goto_check = [
     7,     3,     1,     3,     2,     6,     8,   nil,   nil,   nil,
   nil,     7,   nil,     7,   nil,     7,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,     7 ]

racc_goto_pointer = [
   nil,     2,     4,     1,   nil,   nil,   -10,   -13,   -20 ]

racc_goto_default = [
   nil,   nil,   nil,   nil,     5,     6,   nil,   nil,   nil ]

racc_reduce_table = [
  0, 0, :racc_error,
  2, 16, :_reduce_none,
  1, 16, :_reduce_none,
  2, 17, :_reduce_none,
  1, 17, :_reduce_none,
  1, 18, :_reduce_none,
  1, 18, :_reduce_none,
  6, 19, :_reduce_7,
  5, 19, :_reduce_8,
  3, 21, :_reduce_9,
  1, 21, :_reduce_10,
  3, 20, :_reduce_11,
  4, 22, :_reduce_12,
  3, 22, :_reduce_13,
  4, 22, :_reduce_14,
  2, 22, :_reduce_15,
  1, 22, :_reduce_16,
  1, 22, :_reduce_17,
  1, 22, :_reduce_18,
  1, 22, :_reduce_19,
  1, 22, :_reduce_20,
  3, 23, :_reduce_21,
  1, 23, :_reduce_22 ]

racc_reduce_n = 23

racc_shift_n = 40

racc_token_table = {
  false => 0,
  :error => 1,
  :FN_DEF => 2,
  :ASSIGN => 3,
  "(" => 4,
  ")" => 5,
  "," => 6,
  :END_OF_CODE => 7,
  :EOL => 8,
  :FALSE => 9,
  :ID => 10,
  :NUMBER => 11,
  :STRING => 12,
  :TRUE => 13,
  :VAR_REF => 14 }

racc_nt_base = 15

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
  "FN_DEF",
  "ASSIGN",
  "\"(\"",
  "\")\"",
  "\",\"",
  "END_OF_CODE",
  "EOL",
  "FALSE",
  "ID",
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
  "fn_def_args",
  "exp",
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

module_eval(<<'.,.,', 'code_section.y', 28)
  def _reduce_7(val, _values, result)
     def_function(val[1], val[3], val[5])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 29)
  def _reduce_8(val, _values, result)
     def_function(val[1], [], val[4])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 31)
  def _reduce_9(val, _values, result)
     result = [val[0], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 32)
  def _reduce_10(val, _values, result)
     result = val[0]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 34)
  def _reduce_11(val, _values, result)
     def_variable(val[0], val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 36)
  def _reduce_12(val, _values, result)
     result = Language::FunctionCall.new(val[0], val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 37)
  def _reduce_13(val, _values, result)
     result = Language::FunctionCall.new(val[0], [])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 38)
  def _reduce_14(val, _values, result)
     result = Language::FunctionCall.new(val[0], [val[2]])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 39)
  def _reduce_15(val, _values, result)
     result = Language::Variable.new(val[1])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 40)
  def _reduce_16(val, _values, result)
     result = Language::String.new(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 41)
  def _reduce_17(val, _values, result)
     result = Language::Number.new(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 42)
  def _reduce_18(val, _values, result)
     result = Language::Boolean.new(true)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 43)
  def _reduce_19(val, _values, result)
     result = Language::Boolean.new(false)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 44)
  def _reduce_20(val, _values, result)
     result = val[0]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 46)
  def _reduce_21(val, _values, result)
     result = [val[0], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 47)
  def _reduce_22(val, _values, result)
     result = val[0]
    result
  end
.,.,

def _reduce_none(val, _values, result)
  val[0]
end

    end   # class CodeSectionParser
  end   # module Language
end   # module CSVPlusPlus

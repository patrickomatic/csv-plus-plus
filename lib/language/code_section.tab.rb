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

module_eval(<<'...end code_section.y/module_eval...', 'code_section.y', 59)
  def entities_ns
    ::CSVPlusPlus::Language::Entities
  end

  def def_function(id, arguments, body)
    fn_call = ::CSVPlusPlus::Language::Entities::Function.new(id, arguments, body)
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
      when s.scan(/[!:\w_]+/)
        tokens << [:ID, s.matched]
      when s.scan(/[\(\)\{\}\/\*\+\-,=&]/) # XXX I don't think this is used, get rid of this
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
    32,    28,    29,    36,    37,    22,    17,    20,    19,    21,
    18,     7,     7,    24,     9,    12,     3,    10,    13,    25,
     8,     8,    22,    17,    20,    19,    21,    18,    22,    17,
    20,    19,    21,    18,    22,    17,    20,    19,    21,    18,
    22,    17,    20,    19,    21,    18,    14,    15,    26,    27,
    35,    38 ]

racc_action_check = [
    26,    23,    23,    31,    31,    26,    26,    26,    26,    26,
    26,     0,     2,    15,     1,     7,     0,     2,     8,    15,
     0,     2,    13,    13,    13,    13,    13,    13,    24,    24,
    24,    24,    24,    24,    28,    28,    28,    28,    28,    28,
    37,    37,    37,    37,    37,    37,     9,    12,    17,    18,
    29,    33 ]

racc_action_pointer = [
     9,    14,    10,   nil,   nil,   nil,   nil,     4,    15,    46,
   nil,   nil,    43,    12,   nil,     8,   nil,    44,    38,   nil,
   nil,   nil,   nil,    -4,    18,   nil,    -5,   nil,    24,    39,
   nil,    -2,   nil,    46,   nil,   nil,   nil,    30,   nil,   nil ]

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
  2, 17, :_reduce_none,
  1, 17, :_reduce_none,
  2, 18, :_reduce_none,
  1, 18, :_reduce_none,
  1, 19, :_reduce_none,
  1, 19, :_reduce_none,
  6, 20, :_reduce_7,
  5, 20, :_reduce_8,
  3, 22, :_reduce_9,
  1, 22, :_reduce_10,
  3, 21, :_reduce_11,
  4, 23, :_reduce_12,
  3, 23, :_reduce_13,
  4, 23, :_reduce_14,
  2, 23, :_reduce_15,
  1, 23, :_reduce_16,
  1, 23, :_reduce_17,
  1, 23, :_reduce_18,
  1, 23, :_reduce_19,
  1, 23, :_reduce_20,
  3, 24, :_reduce_21,
  1, 24, :_reduce_22 ]

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
  :CELL_REF => 8,
  :EOL => 9,
  :FALSE => 10,
  :ID => 11,
  :NUMBER => 12,
  :STRING => 13,
  :TRUE => 14,
  :VAR_REF => 15 }

racc_nt_base = 16

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
  "CELL_REF",
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

module_eval(<<'.,.,', 'code_section.y', 29)
  def _reduce_7(val, _values, result)
     def_function(val[1], val[3], val[5])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 30)
  def _reduce_8(val, _values, result)
     def_function(val[1], [], val[4])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 32)
  def _reduce_9(val, _values, result)
     result = [val[0], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 33)
  def _reduce_10(val, _values, result)
     result = val[0]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 35)
  def _reduce_11(val, _values, result)
     def_variable(val[0], val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 37)
  def _reduce_12(val, _values, result)
     result = entities_ns::FunctionCall.new(val[0], val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 38)
  def _reduce_13(val, _values, result)
     result = entities_ns::FunctionCall.new(val[0], [])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 39)
  def _reduce_14(val, _values, result)
     result = entities_ns::FunctionCall.new(val[0], [val[2]])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 40)
  def _reduce_15(val, _values, result)
     result = entities_ns::Variable.new(val[1])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 41)
  def _reduce_16(val, _values, result)
     result = entities_ns::String.new(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 42)
  def _reduce_17(val, _values, result)
     result = entities_ns::Number.new(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 43)
  def _reduce_18(val, _values, result)
     result = entities_ns::Boolean.new(true)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 44)
  def _reduce_19(val, _values, result)
     result = entities_ns::Boolean.new(false)
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 45)
  def _reduce_20(val, _values, result)
     result = entities_ns::CellReference.new(val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 47)
  def _reduce_21(val, _values, result)
     result = [val[0], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'code_section.y', 48)
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

#
# DO NOT MODIFY!!!!
# This file is automatically generated by Racc 1.6.0
# from Racc grammar file "".
#

require 'racc/parser.rb'

require 'strscan'
require_relative 'syntax_error'

module CSVPlusPlus
  class CellValueParser < Racc::Parser

module_eval(<<'...end cell_value.y/module_eval...', 'cell_value.y', 32)
  attr_accessor :ast

  def parse(text)
    return nil unless text.strip.start_with?('=')
    tokens = []

    s = StringScanner.new text
    until s.empty?
      case
      when s.scan(/\s+/)
      when s.scan(/TRUE/)
        tokens << [:TRUE, s.matched]
      when s.scan(/FALSE/) 
        tokens << [:FALSE, s.matched]
      when s.scan(/"(?:[^"\\]|\\(?:["\\\/bfnrt]|u[0-9a-fA-F]{4}))*"/)
        tokens << [:STRING, s.matched]
      when s.scan(/-?[\d.]+/)
        tokens << [:NUMBER, s.matched]
      when s.scan(/[\$\w_]+/)
        tokens << [:ID, s.matched]
      when s.scan(/[\(\)\/\*\+\-,=&]/)
        tokens << [s.matched, s.matched]
      else
        raise SyntaxError.new("Unable to parse starting at", s.peek(100))
      end 
    end
    tokens << [:EOL, :EOL]

    define_singleton_method(:next_token) { tokens.shift }

    begin
      do_parse
    rescue Racc::ParseError => e
      raise SyntaxError.new("Error parsing code section", e.message, 
                    wrapped_error: e, row_number:, cell_number:,)
    end
    @ast
  end
...end cell_value.y/module_eval...
##### State transition tables begin ###

racc_action_table = [
    30,     6,    29,    15,    16,    17,    18,     5,     2,     9,
     8,    10,    11,    31,    14,    15,    16,    17,    18,     6,
    13,    15,    16,    17,    18,     5,    14,     9,     8,    10,
    11,     6,    14,     3,    12,    19,    14,     5,     6,     9,
     8,    10,    11,    14,     5,     6,     9,     8,    10,    11,
   nil,     5,     6,     9,     8,    10,    11,   nil,     5,     6,
     9,     8,    10,    11,   nil,     5,   nil,     9,     8,    10,
    11,     6,    27,    15,    16,    15,    16,     5,   nil,     9,
     8,    10,    11,   nil,    14,     6,    14,    15,    16,    17,
    18,     5,   nil,     9,     8,    10,    11,   nil,    14,    15,
    16,    17,    18,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
    14 ]

racc_action_check = [
    26,     2,    20,    20,    20,    20,    20,     2,     0,     2,
     2,     2,     2,    26,    20,     4,     4,     4,     4,     6,
     4,    21,    21,    21,    21,     6,     4,     6,     6,     6,
     6,    14,    21,     1,     3,     5,    22,    14,    15,    14,
    14,    14,    14,    23,    15,    16,    15,    15,    15,    15,
   nil,    16,    17,    16,    16,    16,    16,   nil,    17,    18,
    17,    17,    17,    17,   nil,    18,   nil,    18,    18,    18,
    18,    19,    19,    24,    24,    25,    25,    19,   nil,    19,
    19,    19,    19,   nil,    24,    31,    25,    28,    28,    28,
    28,    31,   nil,    31,    31,    31,    31,   nil,    28,    32,
    32,    32,    32,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
    32 ]

racc_action_pointer = [
    -6,    33,    -1,    34,    11,    33,    17,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,    29,    36,    43,    50,    57,    69,
    -1,    17,    21,    28,    69,    71,    -3,   nil,    83,   nil,
   nil,    83,    95 ]

racc_action_default = [
   -18,   -18,   -18,   -18,   -18,   -17,   -18,   -10,   -13,   -14,
   -15,   -16,    33,    -1,   -18,   -18,   -18,   -18,   -18,   -18,
   -18,    -4,    -5,    -6,    -7,    -8,   -18,    -3,   -12,    -9,
    -2,   -18,   -11 ]

racc_goto_table = [
     4,     1,    26,   nil,    20,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,    21,    22,    23,    24,    25,    28,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,    32 ]

racc_goto_check = [
     2,     1,     3,   nil,     2,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,     2,     2,     2,     2,     2,     2,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,     2 ]

racc_goto_pointer = [
   nil,     1,    -2,   -17,   nil ]

racc_goto_default = [
   nil,   nil,   nil,   nil,     7 ]

racc_reduce_table = [
  0, 0, :racc_error,
  3, 18, :_reduce_1,
  4, 19, :_reduce_2,
  3, 19, :_reduce_3,
  3, 19, :_reduce_4,
  3, 19, :_reduce_5,
  3, 19, :_reduce_6,
  3, 19, :_reduce_7,
  3, 19, :_reduce_8,
  3, 19, :_reduce_9,
  1, 19, :_reduce_10,
  3, 20, :_reduce_11,
  1, 20, :_reduce_12,
  1, 21, :_reduce_none,
  1, 21, :_reduce_none,
  1, 21, :_reduce_none,
  1, 21, :_reduce_none,
  1, 21, :_reduce_none ]

racc_reduce_n = 18

racc_shift_n = 33

racc_token_table = {
  false => 0,
  :error => 1,
  "(" => 2,
  ")" => 3,
  "*" => 4,
  "/" => 5,
  "+" => 6,
  "-" => 7,
  :ID => 8,
  :EOL => 9,
  :NUMBER => 10,
  :STRING => 11,
  :TRUE => 12,
  :FALSE => 13,
  "=" => 14,
  "&" => 15,
  "," => 16 }

racc_nt_base = 17

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
  "\"*\"",
  "\"/\"",
  "\"+\"",
  "\"-\"",
  "ID",
  "EOL",
  "NUMBER",
  "STRING",
  "TRUE",
  "FALSE",
  "\"=\"",
  "\"&\"",
  "\",\"",
  "$start",
  "cell_value",
  "exp",
  "fn_call_args",
  "literal" ]

Racc_debug_parser = false

##### State transition tables end #####

# reduce 0 omitted

module_eval(<<'.,.,', 'cell_value.y', 8)
  def _reduce_1(val, _values, result)
     @ast = val[1]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 10)
  def _reduce_2(val, _values, result)
     result = [[:fn, val[0]], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 11)
  def _reduce_3(val, _values, result)
     result = [[:fn, val[0]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 12)
  def _reduce_4(val, _values, result)
     result = [[:fn, "CONCAT"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 13)
  def _reduce_5(val, _values, result)
     result = [[:fn, "MULTIPLY"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 14)
  def _reduce_6(val, _values, result)
     result = [[:fn, "DIVIDE"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 15)
  def _reduce_7(val, _values, result)
     result = [[:fn, "ADD"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 16)
  def _reduce_8(val, _values, result)
     result = [[:fn, "MINUS"], [val[0], val[2]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 17)
  def _reduce_9(val, _values, result)
     result = [:group, [val[1]]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 18)
  def _reduce_10(val, _values, result)
     result = [:literal, val[0]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 20)
  def _reduce_11(val, _values, result)
     result = [val[0], val[2]]
    result
  end
.,.,

module_eval(<<'.,.,', 'cell_value.y', 21)
  def _reduce_12(val, _values, result)
     result = val[0]
    result
  end
.,.,

# reduce 13 omitted

# reduce 14 omitted

# reduce 15 omitted

# reduce 16 omitted

# reduce 17 omitted

def _reduce_none(val, _values, result)
  val[0]
end

  end   # class CellValueParser
end   # module CSVPlusPlus

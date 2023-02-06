#
# DO NOT MODIFY!!!!
# This file is automatically generated by Racc 1.6.2
# from Racc grammar file "".
#

require 'racc/parser.rb'

require_relative './expand'
require_relative './lexer'

module CSVPlusPlus
  class ModifierParser < Racc::Parser

module_eval(<<'...end modifier.y/module_eval...', 'modifier.y', 123)
  attr_reader :return_value

  include ::CSVPlusPlus::Lexer

  def initialize(cell_modifier:, row_modifier:)
    super()

    @parsing_row = false
    @cell_modifier = cell_modifier
    @row_modifier = row_modifier
  end

  protected

  def anything_to_parse?(input)
    @modifiers_to_parse = input.scan(/!?\[\[/).count

    if @modifiers_to_parse == 0
      assign_defaults!
      @return_value = input
    end

    @modifiers_to_parse > 0
  end

  def parse_subject
    'modifier'
  end

  def tokenizer(input)
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /\w+/,
      ignore: /\s+/,
      input:,
      stop_fn: lambda do |scanner|
        return false unless scanner.scan(/\]\]/)

        @tokens << [:END_MODIFIERS, scanner.matched]
        @return_value = scanner.rest

        @modifiers_to_parse -= 1
        @modifiers_to_parse == 0
      end,
      tokens: [
        [/\[\[/, :START_CELL_MODIFIERS],
        [/!\[\[/, :START_ROW_MODIFIERS],
        [/^#(([0-9a-fA-F]{2}){3}|([0-9a-fA-F]){3})/, :HEX_COLOR],
        [/(['\w]+\!)?[\w\d]+:[\w\d]+/, :A1_NOTATION],
        [/=/, :EQ],
        [/-?[\d.]+/, :NUMBER],
        [/'(?:[^'\\]|\\(?:['\\\/bfnrt]|u[0-9a-fA-F]{4}))*'/, :STRING],
        [/\//, :MODIFIER_SEPARATOR],
      ],
      alter_matches: {
        STRING: ->(s) { s.gsub(/^'|'$/, '') }
      },
    )
  end

  private

  def assign_defaults!
    @cell_modifier.take_defaults_from!(@row_modifier)
  end

  def parsing_row!
    @parsing_row = true
  end

  def finished_row!
    parsing_cell!
  end

  def parsing_cell!
    @parsing_row = false
    assign_defaults!
  end

  def freeze!
    (@parsing_row ? @row_modifier : @cell_modifier).freeze!
  end

  def s!(property, value)
    target = @parsing_row ? @row_modifier : @cell_modifier
    target.public_send("#{property}=".to_sym, value)
  end
...end modifier.y/module_eval...
##### State transition tables begin ###

racc_action_table = [
   126,    13,    14,    15,    16,    17,    18,    19,    20,    21,
    22,    23,    24,    25,    26,    27,    13,    14,    15,    16,
    17,    18,    19,    20,    21,    22,    23,    24,    25,    26,
    27,    84,    85,    86,    87,    88,    89,    90,    91,     5,
     4,    93,    94,    96,    97,    98,    99,   100,   101,   102,
   103,   104,   106,   107,   108,   109,   110,   111,   112,   113,
   114,   115,   116,   117,   118,   119,   120,   121,   122,   123,
   124,   125,   127,   128,   129,   130,   131,   132,    13,    14,
    15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
    25,    26,    27,    29,    45,    50,   136,    52,    30,    30,
    60,     6,    59,    58,    61,    57,    64,    65,    66,    67,
    68,    69,    60,     5,    59,    58,    61,    57,   127,   128,
   129,   130,   131,   132,   127,   128,   129,   130,   131,   132,
    50,    51,    52,    53,    54,    78,    79,    80,    81,   134,
    10,    53,    54,    78,    79,    80,    81,   -40,    31,   -40,
    32,    33,    34,    35,    36,    37,    38,    39,    40,    41,
    42,    43,    44,    62,    70,    71,    72,    73,    74,    75,
    82,   139,   140,   141,   142,   143,   144,   145,   146,   147,
   148,   149,   150,   151,   152,   153,   154,   155,   156,   157,
   158,   159,   160,   161,   162,   163,   164,   126,   126,   126,
   126,   126,   126,   126,   126,   126,   126,   126,   126,   126,
   126,   126,   126,   126,   185,   126,   126,   126,   126,   126,
   126,   126,   126,   126,   126,   126,   126,   126 ]

racc_action_check = [
    44,     8,     8,     8,     8,     8,     8,     8,     8,     8,
     8,     8,     8,     8,     8,     8,     9,     9,     9,     9,
     9,     9,     9,     9,     9,     9,     9,     9,     9,     9,
     9,    43,    43,    43,    43,    43,    43,    43,    43,     0,
     0,    44,    44,    44,    44,    44,    44,    44,    44,    44,
    44,    44,    44,    44,    44,    44,    44,    44,    44,    44,
    44,    44,    44,    44,    44,    44,    44,    44,    44,    44,
    44,    44,    44,    44,    44,    44,    44,    44,    30,    30,
    30,    30,    30,    30,    30,    30,    30,    30,    30,    30,
    30,    30,    30,    11,    28,    49,    49,    49,    11,    28,
    32,     1,    32,    32,    32,    32,    34,    34,    34,    34,
    34,    34,    55,     2,    55,    55,    55,    55,   141,   141,
   141,   141,   141,   141,   142,   142,   142,   142,   142,   142,
    31,    31,    31,    31,    31,    41,    41,    41,    41,    48,
     6,    48,    48,    76,    76,    76,    76,    51,    13,    51,
    14,    15,    16,    17,    18,    19,    20,    21,    22,    23,
    25,    26,    27,    33,    35,    36,    37,    38,    39,    40,
    42,    94,    96,    97,    98,    99,   100,   102,   103,   104,
   106,   108,   109,   110,   111,   112,   113,   114,   115,   116,
   117,   118,   119,   120,   123,   124,   125,   139,   140,   143,
   144,   145,   146,   147,   148,   149,   150,   151,   152,   153,
   154,   155,   156,   157,   158,   159,   160,   161,   162,   163,
   164,   169,   171,   172,   176,   182,   184,   190 ]

racc_action_pointer = [
    27,   101,   101,   nil,   nil,   nil,   140,   nil,   -15,     0,
   nil,    87,   nil,   141,   143,   144,   145,   146,   147,   148,
   149,   150,   151,   152,   nil,   153,   154,   155,    88,   nil,
    62,    95,    65,   155,    65,   156,   156,   152,   159,   154,
   160,   104,   156,   -16,   -14,   nil,   nil,   nil,   103,    60,
   nil,   112,   nil,   nil,   nil,    77,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   112,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   169,   nil,   170,   171,   172,   173,
   174,   nil,   175,   176,   177,   nil,   178,   nil,   179,   180,
   181,   182,   183,   184,   185,   186,   187,   188,   189,   190,
   191,   nil,   nil,   192,   193,   194,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   183,
   184,    32,    38,   185,   186,   187,   188,   189,   190,   191,
   192,   193,   194,   195,   196,   197,   198,   199,   209,   201,
   202,   203,   204,   205,   206,   nil,   nil,   nil,   nil,   207,
   nil,   208,   209,   nil,   nil,   nil,   210,   nil,   nil,   nil,
   nil,   nil,   211,   nil,   212,   nil,   nil,   nil,   nil,   nil,
   213,   nil,   nil,   nil,   nil,   nil,   nil ]

racc_action_default = [
  -108,  -108,    -2,    -3,    -4,    -6,  -108,    -1,  -108,  -108,
   197,  -108,    -9,  -108,  -108,  -108,  -108,  -108,   -16,  -108,
  -108,  -108,  -108,  -108,   -22,  -108,  -108,  -108,  -108,    -5,
  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,
  -108,  -108,  -108,  -108,  -108,    -7,    -8,   -10,   -34,   -35,
   -36,   -37,   -38,   -39,   -41,   -11,   -43,   -44,   -45,   -46,
   -47,   -48,   -12,   -13,   -49,   -50,   -51,   -52,   -53,   -54,
   -14,   -15,   -17,   -18,   -19,   -20,   -21,   -27,   -28,   -29,
   -30,   -31,   -23,   -24,   -55,   -56,   -57,   -58,   -59,   -60,
   -61,   -62,   -25,   -63,   -64,   -67,  -108,  -108,  -108,  -108,
  -108,   -73,  -108,  -108,  -108,   -77,  -108,   -80,  -108,  -108,
  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,
  -108,   -94,   -95,  -108,  -108,  -108,  -101,  -102,  -103,  -104,
  -105,  -106,  -107,   -32,   -40,   -33,   -37,   -42,   -26,  -108,
  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,
  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,  -108,
  -108,  -108,  -108,  -108,  -108,   -65,   -68,   -69,   -70,  -108,
   -72,  -108,   -75,  -100,   -76,   -78,  -108,   -82,   -83,   -84,
   -85,   -86,  -108,   -88,   -89,   -90,   -91,   -92,   -93,   -96,
   -97,   -98,   -71,   -74,   -99,   -81,   -87 ]

racc_goto_table = [
    95,    77,    56,    48,     1,    49,   172,     3,     2,     7,
    11,    28,     8,     9,    46,    47,   105,   184,    55,    63,
    76,   135,   133,   190,    83,   137,    92,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   138,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   165,   166,   nil,   nil,   169,
   170,   171,   nil,   174,   175,   176,   177,   178,   179,   180,
   181,   182,   183,   167,   168,   186,   187,   188,   189,   nil,
   191,   nil,   nil,   nil,   nil,   192,   nil,   193,   194,   nil,
   nil,   nil,   195,   nil,   nil,   nil,   nil,   nil,   196,   nil,
   194,   nil,   nil,   nil,   nil,   nil,   194 ]

racc_goto_check = [
    18,    14,    17,    15,     1,    16,    20,     3,     2,     3,
     4,     4,     5,     6,     7,     8,    19,    20,     9,    10,
    11,    15,    16,    20,    12,    17,    13,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,    14,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,    18,    18,   nil,   nil,    18,
    18,    18,   nil,    18,    18,    18,    18,    18,    18,    18,
    18,    18,    18,    19,    19,    18,    18,    18,    18,   nil,
    18,   nil,   nil,   nil,   nil,    18,   nil,    18,    18,   nil,
   nil,   nil,    18,   nil,   nil,   nil,   nil,   nil,    18,   nil,
    18,   nil,   nil,   nil,   nil,   nil,    18 ]

racc_goto_pointer = [
   nil,     4,     8,     7,     2,     8,     8,   -16,   -16,   -14,
   -15,   -21,   -19,   -18,   -40,   -28,   -26,   -30,   -44,   -28,
  -140 ]

racc_goto_default = [
   nil,   nil,   nil,   nil,   nil,   nil,   nil,    12,   nil,   nil,
   nil,   nil,   nil,   nil,   nil,   nil,   nil,   nil,   173,   nil,
   nil ]

racc_reduce_table = [
  0, 0, :racc_error,
  2, 93, :_reduce_none,
  1, 93, :_reduce_none,
  1, 93, :_reduce_none,
  0, 97, :_reduce_4,
  4, 94, :_reduce_5,
  0, 98, :_reduce_6,
  4, 95, :_reduce_none,
  3, 96, :_reduce_none,
  1, 96, :_reduce_none,
  3, 99, :_reduce_none,
  3, 99, :_reduce_none,
  3, 99, :_reduce_12,
  3, 99, :_reduce_13,
  3, 99, :_reduce_14,
  3, 99, :_reduce_15,
  1, 99, :_reduce_16,
  3, 99, :_reduce_17,
  3, 99, :_reduce_18,
  3, 99, :_reduce_19,
  3, 99, :_reduce_20,
  3, 99, :_reduce_none,
  1, 99, :_reduce_22,
  3, 99, :_reduce_23,
  3, 99, :_reduce_24,
  3, 99, :_reduce_25,
  2, 103, :_reduce_none,
  1, 103, :_reduce_27,
  1, 106, :_reduce_none,
  1, 106, :_reduce_none,
  1, 106, :_reduce_none,
  1, 106, :_reduce_none,
  2, 100, :_reduce_32,
  2, 100, :_reduce_33,
  1, 100, :_reduce_34,
  1, 100, :_reduce_35,
  1, 107, :_reduce_none,
  1, 107, :_reduce_none,
  1, 107, :_reduce_none,
  1, 108, :_reduce_none,
  1, 108, :_reduce_none,
  1, 108, :_reduce_none,
  2, 101, :_reduce_none,
  1, 101, :_reduce_43,
  1, 109, :_reduce_none,
  1, 109, :_reduce_none,
  1, 109, :_reduce_none,
  1, 109, :_reduce_none,
  1, 109, :_reduce_none,
  1, 102, :_reduce_none,
  1, 102, :_reduce_none,
  1, 102, :_reduce_none,
  1, 102, :_reduce_none,
  1, 102, :_reduce_none,
  1, 102, :_reduce_none,
  1, 104, :_reduce_none,
  1, 104, :_reduce_none,
  1, 104, :_reduce_none,
  1, 104, :_reduce_none,
  1, 104, :_reduce_none,
  1, 104, :_reduce_none,
  1, 104, :_reduce_none,
  1, 104, :_reduce_none,
  1, 105, :_reduce_none,
  1, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  1, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  4, 105, :_reduce_none,
  3, 105, :_reduce_none,
  1, 105, :_reduce_none,
  4, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  1, 105, :_reduce_none,
  3, 105, :_reduce_none,
  1, 105, :_reduce_none,
  1, 105, :_reduce_none,
  4, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  4, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  1, 105, :_reduce_none,
  1, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  3, 105, :_reduce_none,
  2, 112, :_reduce_none,
  1, 112, :_reduce_none,
  1, 110, :_reduce_none,
  1, 111, :_reduce_none,
  1, 111, :_reduce_none,
  1, 111, :_reduce_none,
  1, 111, :_reduce_none,
  1, 111, :_reduce_none,
  1, 111, :_reduce_none ]

racc_reduce_n = 108

racc_shift_n = 197

racc_token_table = {
  false => 0,
  :error => 1,
  ":" => 2,
  "=" => 3,
  "/" => 4,
  :A1_NOTATION => 5,
  :END_MODIFIERS => 6,
  :EQ => 7,
  :HEX_COLOR => 8,
  :NUMBER => 9,
  :MODIFIER_ID => 10,
  :MODIFIER_SEPARATOR => 11,
  :START_CELL_MODIFIERS => 12,
  :START_ROW_MODIFIERS => 13,
  :STRING => 14,
  :URL => 15,
  "align" => 16,
  "border" => 17,
  "bordercolor" => 18,
  "borderstyle" => 19,
  "color" => 20,
  "expand" => 21,
  "font" => 22,
  "fontcolor" => 23,
  "fontfamily" => 24,
  "fontsize" => 25,
  "format" => 26,
  "freeze" => 27,
  "note" => 28,
  "numberformat" => 29,
  "validate" => 30,
  "bold" => 31,
  "italic" => 32,
  "strikethrough" => 33,
  "underline" => 34,
  "left" => 35,
  "center" => 36,
  "right" => 37,
  "top" => 38,
  "bottom" => 39,
  "all" => 40,
  "dashed" => 41,
  "dotted" => 42,
  "double" => 43,
  "solid" => 44,
  "solid_medium" => 45,
  "solid_thick" => 46,
  "currency" => 47,
  "date" => 48,
  "date_time" => 49,
  "number" => 50,
  "percent" => 51,
  "text" => 52,
  "time" => 53,
  "scientific" => 54,
  "blank" => 55,
  "boolean" => 56,
  "custom_formula" => 57,
  "date_after" => 58,
  "date_before" => 59,
  "date_between" => 60,
  "date_eq" => 61,
  "date_is_valid" => 62,
  "date_not_between" => 63,
  "date_not_eq" => 64,
  "date_on_or_after" => 65,
  "date_on_or_before" => 66,
  "not_blank" => 67,
  "number_between" => 68,
  "number_eq" => 69,
  "number_greater" => 70,
  "number_greater_than_eq" => 71,
  "number_less" => 72,
  "number_less_than_eq" => 73,
  "number_not_between" => 74,
  "number_not_eq" => 75,
  "one_of_list" => 76,
  "one_of_range" => 77,
  "text_contains" => 78,
  "text_ends_with" => 79,
  "text_eq" => 80,
  "text_is_email" => 81,
  "text_is_url" => 82,
  "text_not_contains" => 83,
  "text_not_eq" => 84,
  "text_starts_with" => 85,
  "past_year" => 86,
  "past_month" => 87,
  "past_week" => 88,
  "yesterday" => 89,
  "today" => 90,
  "tomorrow" => 91 }

racc_nt_base = 92

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
  "\":\"",
  "\"=\"",
  "\"/\"",
  "A1_NOTATION",
  "END_MODIFIERS",
  "EQ",
  "HEX_COLOR",
  "NUMBER",
  "MODIFIER_ID",
  "MODIFIER_SEPARATOR",
  "START_CELL_MODIFIERS",
  "START_ROW_MODIFIERS",
  "STRING",
  "URL",
  "\"align\"",
  "\"border\"",
  "\"bordercolor\"",
  "\"borderstyle\"",
  "\"color\"",
  "\"expand\"",
  "\"font\"",
  "\"fontcolor\"",
  "\"fontfamily\"",
  "\"fontsize\"",
  "\"format\"",
  "\"freeze\"",
  "\"note\"",
  "\"numberformat\"",
  "\"validate\"",
  "\"bold\"",
  "\"italic\"",
  "\"strikethrough\"",
  "\"underline\"",
  "\"left\"",
  "\"center\"",
  "\"right\"",
  "\"top\"",
  "\"bottom\"",
  "\"all\"",
  "\"dashed\"",
  "\"dotted\"",
  "\"double\"",
  "\"solid\"",
  "\"solid_medium\"",
  "\"solid_thick\"",
  "\"currency\"",
  "\"date\"",
  "\"date_time\"",
  "\"number\"",
  "\"percent\"",
  "\"text\"",
  "\"time\"",
  "\"scientific\"",
  "\"blank\"",
  "\"boolean\"",
  "\"custom_formula\"",
  "\"date_after\"",
  "\"date_before\"",
  "\"date_between\"",
  "\"date_eq\"",
  "\"date_is_valid\"",
  "\"date_not_between\"",
  "\"date_not_eq\"",
  "\"date_on_or_after\"",
  "\"date_on_or_before\"",
  "\"not_blank\"",
  "\"number_between\"",
  "\"number_eq\"",
  "\"number_greater\"",
  "\"number_greater_than_eq\"",
  "\"number_less\"",
  "\"number_less_than_eq\"",
  "\"number_not_between\"",
  "\"number_not_eq\"",
  "\"one_of_list\"",
  "\"one_of_range\"",
  "\"text_contains\"",
  "\"text_ends_with\"",
  "\"text_eq\"",
  "\"text_is_email\"",
  "\"text_is_url\"",
  "\"text_not_contains\"",
  "\"text_not_eq\"",
  "\"text_starts_with\"",
  "\"past_year\"",
  "\"past_month\"",
  "\"past_week\"",
  "\"yesterday\"",
  "\"today\"",
  "\"tomorrow\"",
  "$start",
  "modifiers_definition",
  "row_modifiers",
  "cell_modifiers",
  "modifiers",
  "@1",
  "@2",
  "modifier",
  "align_options",
  "border_options",
  "borderstyle_option",
  "format_options",
  "numberformat_option",
  "condition",
  "format_option",
  "halign_option",
  "valign_option",
  "border_option",
  "condition_value",
  "relative_date",
  "condition_values" ]

Racc_debug_parser = false

##### State transition tables end #####

# reduce 0 omitted

# reduce 1 omitted

# reduce 2 omitted

# reduce 3 omitted

module_eval(<<'.,.,', 'modifier.y', 24)
  def _reduce_4(val, _values, result)
     parsing_row!
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 26)
  def _reduce_5(val, _values, result)
     finished_row!
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 28)
  def _reduce_6(val, _values, result)
     parsing_cell!
    result
  end
.,.,

# reduce 7 omitted

# reduce 8 omitted

# reduce 9 omitted

# reduce 10 omitted

# reduce 11 omitted

module_eval(<<'.,.,', 'modifier.y', 36)
  def _reduce_12(val, _values, result)
     s!(:bordercolor, val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 37)
  def _reduce_13(val, _values, result)
     s!(:borderstyle, val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 38)
  def _reduce_14(val, _values, result)
     s!(:color, val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 39)
  def _reduce_15(val, _values, result)
     s!(:expand, Expand.new(val[2].to_i))
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 40)
  def _reduce_16(val, _values, result)
     s!(:expand, Expand.new)
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 41)
  def _reduce_17(val, _values, result)
     s!(:fontfamily, val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 42)
  def _reduce_18(val, _values, result)
     s!(:fontcolor, val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 43)
  def _reduce_19(val, _values, result)
     s!(:fontfamily, val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 44)
  def _reduce_20(val, _values, result)
     s!(:fontsize, val[2].to_f)
    result
  end
.,.,

# reduce 21 omitted

module_eval(<<'.,.,', 'modifier.y', 46)
  def _reduce_22(val, _values, result)
     freeze!
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 47)
  def _reduce_23(val, _values, result)
     s!(:note, val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 48)
  def _reduce_24(val, _values, result)
     s!(:numberformat, val[2])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 49)
  def _reduce_25(val, _values, result)
     s!(:validation, val[2])
    result
  end
.,.,

# reduce 26 omitted

module_eval(<<'.,.,', 'modifier.y', 51)
  def _reduce_27(val, _values, result)
     s!(:format, val[0])
    result
  end
.,.,

# reduce 28 omitted

# reduce 29 omitted

# reduce 30 omitted

# reduce 31 omitted

module_eval(<<'.,.,', 'modifier.y', 54)
  def _reduce_32(val, _values, result)
     s!(:align, val[0]); s!(:align, val[1])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 55)
  def _reduce_33(val, _values, result)
     s!(:align, val[0]); s!(:align, val[1])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 56)
  def _reduce_34(val, _values, result)
     s!(:align, val[0])
    result
  end
.,.,

module_eval(<<'.,.,', 'modifier.y', 57)
  def _reduce_35(val, _values, result)
     s!(:align, val[0])
    result
  end
.,.,

# reduce 36 omitted

# reduce 37 omitted

# reduce 38 omitted

# reduce 39 omitted

# reduce 40 omitted

# reduce 41 omitted

# reduce 42 omitted

module_eval(<<'.,.,', 'modifier.y', 62)
  def _reduce_43(val, _values, result)
     s!(:border, val[0])
    result
  end
.,.,

# reduce 44 omitted

# reduce 45 omitted

# reduce 46 omitted

# reduce 47 omitted

# reduce 48 omitted

# reduce 49 omitted

# reduce 50 omitted

# reduce 51 omitted

# reduce 52 omitted

# reduce 53 omitted

# reduce 54 omitted

# reduce 55 omitted

# reduce 56 omitted

# reduce 57 omitted

# reduce 58 omitted

# reduce 59 omitted

# reduce 60 omitted

# reduce 61 omitted

# reduce 62 omitted

# reduce 63 omitted

# reduce 64 omitted

# reduce 65 omitted

# reduce 66 omitted

# reduce 67 omitted

# reduce 68 omitted

# reduce 69 omitted

# reduce 70 omitted

# reduce 71 omitted

# reduce 72 omitted

# reduce 73 omitted

# reduce 74 omitted

# reduce 75 omitted

# reduce 76 omitted

# reduce 77 omitted

# reduce 78 omitted

# reduce 79 omitted

# reduce 80 omitted

# reduce 81 omitted

# reduce 82 omitted

# reduce 83 omitted

# reduce 84 omitted

# reduce 85 omitted

# reduce 86 omitted

# reduce 87 omitted

# reduce 88 omitted

# reduce 89 omitted

# reduce 90 omitted

# reduce 91 omitted

# reduce 92 omitted

# reduce 93 omitted

# reduce 94 omitted

# reduce 95 omitted

# reduce 96 omitted

# reduce 97 omitted

# reduce 98 omitted

# reduce 99 omitted

# reduce 100 omitted

# reduce 101 omitted

# reduce 102 omitted

# reduce 103 omitted

# reduce 104 omitted

# reduce 105 omitted

# reduce 106 omitted

# reduce 107 omitted

def _reduce_none(val, _values, result)
  val[0]
end

  end   # class ModifierParser
end   # module CSVPlusPlus
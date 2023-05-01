class CSVPlusPlus::Parser::Modifier

prechigh
  left '![['
  left '[[' ']]'
  left '='
  left '/'
preclow

token END_MODIFIERS
      EQ
      HEX_COLOR
      NUMBER
      MODIFIER
      MODIFIER_SEPARATOR
      RIGHT_SIDE
      START_CELL_MODIFIERS
      START_ROW_MODIFIERS
      STRING

rule
  modifiers_definition: row_modifiers cell_modifiers 
                      | row_modifiers 
                      | cell_modifiers

  row_modifiers: START_ROW_MODIFIERS   { parsing_row! }
                 modifiers
                 END_MODIFIERS         { finished_row! }

  cell_modifiers: START_CELL_MODIFIERS { parsing_cell! }
                  modifiers
                  END_MODIFIERS

  modifiers: modifiers MODIFIER_SEPARATOR modifier | modifier

  modifier: 'border'       EQ RIGHT_SIDE  { modifier.border = val[2]        }
          | 'bordercolor'  EQ HEX_COLOR   { modifier.bordercolor = val[2]   }
          | 'borderstyle'  EQ RIGHT_SIDE  { modifier.borderstyle = val[2]   }
          | 'color'        EQ HEX_COLOR   { modifier.color = val[2]         }
          | 'expand'       EQ NUMBER      { modifier.expand = val[2]        }
          | 'expand'                      { modifier.infinite_expand!       }
          | 'fontcolor'    EQ HEX_COLOR   { modifier.fontcolor = val[2]     }
          | 'fontfamily'   EQ RIGHT_SIDE  { modifier.fontfamily = val[2]    }
          | 'fontsize'     EQ NUMBER      { modifier.fontsize = val[2]      }
          | 'format'       EQ RIGHT_SIDE  { modifier.format = val[2]        }
          | 'freeze'                      { modifier.freeze!                }
          | 'halign'       EQ RIGHT_SIDE  { modifier.halign = val[2]        }
          | 'note'         EQ RIGHT_SIDE  { modifier.note = val[2]          }
          | 'numberformat' EQ RIGHT_SIDE  { modifier.numberformat = val[2]  }
          | 'validate'     EQ RIGHT_SIDE  { modifier.validate = val[2]      }
          | 'valign'       EQ RIGHT_SIDE  { modifier.valign = val[2]        }
          | 'var'          EQ RIGHT_SIDE  { modifier.var = val[2]           }
end

---- header

require_relative '../lexer/racc_lexer'

---- inner
  extend ::T::Sig
  extend ::T::Generic
  include ::CSVPlusPlus::Lexer::RaccLexer

  ReturnType = type_member {{ fixed: ::T.nilable(::String) }}

  # @param cell_modifier [Modifier]
  # @param row_modifier [Modifier]
  def initialize(cell_modifier:, row_modifier:)
    super()

    @parsing_row = false
    @cell_modifier = ::CSVPlusPlus::Modifier::ModifierValidator.new(cell_modifier)
    @row_modifier = ::CSVPlusPlus::Modifier::ModifierValidator.new(row_modifier)
  end

  protected

  sig { override.params(input: ::String).returns(::T::Boolean) }
  def anything_to_parse?(input)
    @modifiers_to_parse = input.scan(/!?\[\[/).count

    if @modifiers_to_parse == 0
      assign_defaults!
      @return_value = input
    end

    @modifiers_to_parse > 0
  end

  sig { override.returns(::String) }
  def parse_subject
    'modifier'
  end

  sig { override.returns(ReturnType) }
  # The output of the parser
  def return_value
    @return_value
  end

  sig { override.returns(::CSVPlusPlus::Lexer::Tokenizer) }
  def tokenizer
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      ignore: /\s+/,
      stop_fn: lambda do |scanner|
        return false unless scanner.scan(/\]\]/)

        @tokens << [:END_MODIFIERS, scanner.matched]
        @return_value = scanner.rest

        @modifiers_to_parse -= 1
        @modifiers_to_parse == 0
      end,
      tokens: [
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bborder\b/, token: 'border'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bbordercolor\b/, token: 'bordercolor'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bborderstyle\b/, token: 'borderstyle'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bcolor\b/, token: 'color'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bexpand\b/, token: 'expand'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bfontcolor\b/, token: 'fontcolor'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bfontfamily\b/, token: 'fontfamily'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bfontsize\b/, token: 'fontsize'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bformat\b/, token: 'format'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bfreeze\b/, token: 'freeze'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bhalign\b/, token: 'halign'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bnote\b/, token: 'note'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bnumberformat\b/, token: 'numberformat'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bvalidate\b/, token: 'validate'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bvalign\b/, token: 'valign'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bvar\b/, token: 'var'),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /-?[\d.]+/, token: :NUMBER),
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:HEX_COLOR],
        ::CSVPlusPlus::Lexer::Token.new(
          regexp: /
            (?:
              \w+\s*:\s*'([^'\\]|\\.)*')    # allow for a single-quoted string which can accept any input and also allow 
                                            # for escaping via backslash (i.e., 'ain\\'t won\\'t something' is valid)
              |                             # - or -
            (?:'([^'\\]|\\.)*')             # allow for a single-quoted string which can accept any input and also allow 
              |
            (?:
              [\w,_:-]                      # something that accepts most basic input if it doesn't need to be quoted
              [\w\s,_:-]+                   # same thing but allow spaces in the middle
              [\w,_:-]                      # no spaces at the end
            )
          /x,
          token: :RIGHT_SIDE,
        ),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\[\[/, token: :START_CELL_MODIFIERS),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /!\[\[/, token: :START_ROW_MODIFIERS),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\//, token: :MODIFIER_SEPARATOR),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /=/, token: :EQ),
      ],
      alter_matches: {
        STRING: ->(s) { s.gsub(/^'|'$/, '') }
      },
    )
  end

  private

  def assign_defaults!
    @cell_modifier.modifier.take_defaults_from!(@row_modifier.modifier)
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

  def modifier
    @parsing_row ? @row_modifier : @cell_modifier
  end

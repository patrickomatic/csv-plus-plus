class CSVPlusPlus::ModifierParser
prechigh
  left '![['
  left '[[' ']]'
  left '='
  left '/'
preclow

token A1_NOTATION
      END_MODIFIERS
      EQ
      HEX_COLOR
      NUMBER
      MODIFIER
      MODIFIER_SEPARATOR
      START_CELL_MODIFIERS
      START_ROW_MODIFIERS
      STRING
      WORD

rule
  modifiers_definition: row_modifiers cell_modifiers 
                      | row_modifiers 
                      | cell_modifiers

  row_modifiers: START_ROW_MODIFIERS      { parsing_row! }
                 modifiers 
                 END_MODIFIERS            { finished_row! }

  cell_modifiers: START_CELL_MODIFIERS    { parsing_cell! }
                  modifiers 
                  END_MODIFIERS 

  modifiers: modifiers MODIFIER_SEPARATOR modifier | modifier

  modifier: 'border'       EQ WORD        { modifier.border = val[2]        }
          | 'bordercolor'  EQ HEX_COLOR   { modifier.bordercolor = val[2]   }
          | 'borderstyle'  EQ WORD        { modifier.borderstyle = val[2]   }
          | 'color'        EQ HEX_COLOR   { modifier.color = val[2]         }
          | 'expand'       EQ NUMBER      { modifier.expand = val[2]        }
          | 'expand'                      { modifier.expand!                }
          | 'fontcolor'    EQ HEX_COLOR   { modifier.fontcolor = val[2]     }
          | 'fontfamily'   EQ STRING      { modifier.fontfamily = val[2]    }
          | 'fontsize'     EQ NUMBER      { modifier.fontsize = val[2]      }
          | 'format'       EQ WORD        { modifier.format = val[2]        }
          | 'freeze'                      { modifier.freeze!                }
          | 'halign'       EQ WORD        { modifier.halign = val[2]        }
          | 'note'         EQ STRING      { modifier.note = val[2]          }
          | 'numberformat' EQ WORD        { modifier.numberformat = val[2]  }
          | 'validate'     EQ WORD        { modifier.validation = val[2]    }
          | 'valign'       EQ WORD        { modifier.valign = val[2]        }
          | 'var'          EQ WORD        { define_var(val[2])              }
end

---- header
require_relative '../expand'
require_relative '../lexer'

---- inner
  attr_reader :return_value

  include ::CSVPlusPlus::Lexer

  # @param cell_modifier [Modifier]
  # @param row_modifier [Modifier]
  # @param scope [Scope]
  def initialize(cell_modifier:, row_modifier:, scope:)
    super()

    @parsing_row = false
    @cell_modifier = cell_modifier
    @row_modifier = row_modifier
    @scope = scope
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
        [/\bborder\b/, 'border'],
        [/\bbordercolor\b/, 'bordercolor'],
        [/\bborderstyle\b/, 'borderstyle'],
        [/\bcolor\b/, 'color'],
        [/\bexpand\b/, 'expand'],
        [/\bfontcolor\b/, 'fontcolor'],
        [/\bfontfamily\b/, 'fontfamily'],
        [/\bfontsize\b/, 'fontsize'],
        [/\bformat\b/, 'format'],
        [/\bfreeze\b/, 'freeze'],
        [/\bhalign\b/, 'halign'],
        [/\bnote\b/, 'note'],
        [/\bnumberformat\b/, 'numberformat'],
        [/\bvalidate\b/, 'validate'],
        [/\bvalign\b/, 'valign'],
        [/\bvar\b/, 'var'],
        [/\[\[/, :START_CELL_MODIFIERS],
        [/!\[\[/, :START_ROW_MODIFIERS],
        [/=/, :EQ],
        [/-?[\d.]+/, :NUMBER],
        [/'(?:[^'\\]|\\(?:['\\\/bfnrt]|u[0-9a-fA-F]{4}))*'/, :STRING],
        [/\//, :MODIFIER_SEPARATOR],
        TOKEN_LIBRARY[:A1_NOTATION],
        TOKEN_LIBRARY[:HEX_COLOR],
        [/[\w]+/, :WORD],
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

  def define_var(var_id)
    @scope.bind_variable_to_cell(var_id)
    modifier.var = var_id.to_sym
  end

  def modifier
    @parsing_row ? @row_modifier : @cell_modifier
  end

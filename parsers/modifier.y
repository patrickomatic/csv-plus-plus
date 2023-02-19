class CSVPlusPlus::ModifierParser
prechigh
  left '![['
  left '[[' ']]'
  left ':'
  left '='
  left '/'
preclow

token A1_NOTATION
      END_MODIFIERS
      EQ
      HEX_COLOR
      NUMBER
      MODIFIER_ID
      MODIFIER_SEPARATOR
      START_CELL_MODIFIERS
      START_ROW_MODIFIERS
      STRING
      URL

rule
  modifiers_definition: row_modifiers cell_modifiers 
                      | row_modifiers 
                      | cell_modifiers

  row_modifiers: START_ROW_MODIFIERS    { parsing_row! }
                 modifiers 
                 END_MODIFIERS          { finished_row! }

  cell_modifiers: START_CELL_MODIFIERS  { parsing_cell! }
                  modifiers 
                  END_MODIFIERS 

  modifiers: modifiers MODIFIER_SEPARATOR modifier | modifier

  modifier: 'halign'       EQ halign_option       { s!(:halign, val[2])                    }
          | 'valign'       EQ valign_option       { s!(:valign, val[2])                    }
          | 'border'       EQ border_options
          | 'bordercolor'  EQ HEX_COLOR           { s!(:bordercolor, val[2])               }
          | 'borderstyle'  EQ borderstyle_option  { s!(:borderstyle, val[2])               }
          | 'color'        EQ HEX_COLOR           { s!(:color, val[2])                     }
          | 'expand'       EQ NUMBER              { s!(:expand, Expand.new(val[2].to_i))   }
          | 'expand'                              { s!(:expand, Expand.new)                }
          | 'font'         EQ STRING              { s!(:fontfamily, val[2])                }
          | 'fontcolor'    EQ HEX_COLOR           { s!(:fontcolor, val[2])                 }
          | 'fontfamily'   EQ STRING              { s!(:fontfamily, val[2])                }
          | 'fontsize'     EQ NUMBER              { s!(:fontsize, val[2].to_f)             }
          | 'format'       EQ format_options
          | 'freeze'                              { freeze!                                }
          | 'note'         EQ STRING              { s!(:note, val[2])                      }
          | 'numberformat' EQ numberformat_option { s!(:numberformat, val[2])              }
          | 'validate'     EQ condition           { s!(:validation, val[2])                }

  format_options: format_options format_option | format_option { s!(:format, val[0])       }
  format_option: 'bold' | 'italic' | 'strikethrough' | 'underline'

  halign_option: 'left' | 'center' | 'right'
  valign_option: 'top'  | 'center' | 'bottom'

  border_options: border_options border_option | border_option { s!(:border, val[0])       }
  border_option: 'all' | 'top' | 'right' | 'left' | 'bottom'

  borderstyle_option: 'dashed' | 'dotted' | 'double' | 'solid' | 'solid_medium' | 'solid_thick'

  numberformat_option: 'currency'
                     | 'date'
                     | 'date_time'
                     | 'number'
                     | 'percent'
                     | 'text'
                     | 'time'
                     | 'scientific'

  condition: 'blank'
           | 'boolean'
           | 'boolean'                 ':' condition_value
           | 'boolean'                 ':' condition_value | condition_value
           | 'custom_formula'          ':' condition_value
           | 'date_after'              ':' relative_date
           | 'date_before'             ':' relative_date
           | 'date_between'            ':' condition_value condition_value
           | 'date_eq'                 ':' condition_value
           | 'date_is_valid'
           | 'date_not_between'        ':' condition_value condition_value
           | 'date_not_eq'             ':' condition_values
           | 'date_on_or_after'        ':' condition_value | relative_date
           | 'date_on_or_before'       ':' condition_value | relative_date
           | 'not_blank'
           | 'number_between'          ':' condition_value condition_value
           | 'number_eq'               ':' condition_value
           | 'number_greater'          ':' condition_value
           | 'number_greater_than_eq'  ':' condition_value
           | 'number_less'             ':' condition_value
           | 'number_less_than_eq'     ':' condition_value
           | 'number_not_between'      ':' condition_value condition_value
           | 'number_not_eq'           ':' condition_value
           | 'one_of_list'             ':' condition_values
           | 'one_of_range'            ':' A1_NOTATION
           | 'text_contains'           ':' condition_value
           | 'text_ends_with'          ':' condition_value
           | 'text_eq'                 ':' condition_value
           | 'text_is_email'
           | 'text_is_url'
           | 'text_not_contains'       ':' condition_value
           | 'text_not_eq'             ':' condition_values
           | 'text_starts_with'        ':' condition_value

  condition_values: condition_values condition_value | condition_value
  condition_value: STRING

  relative_date: 'past_year' | 'past_month' | 'past_week' | 'yesterday' | 'today' | 'tomorrow'

end

---- header
require_relative './expand'
require_relative './lexer'

---- inner
  attr_reader :return_value

  include ::CSVPlusPlus::Lexer

  # @param cell_modifier 
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

  def tokenizer
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /\w+/,
      ignore: /\s+/,
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

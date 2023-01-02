class CSVPlusPlus::ModifierParser
prechigh
  left ':'
  left '='
  left '/'
preclow

token A1_NOTATION
      END_MODIFIERS
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

  modifier: 'align'       '=' align_options
          | 'border'      '=' border_options
          | 'bordercolor' '=' HEX_COLOR           { s!(:bordercolor, val[2])                        }
          | 'borderstyle' '=' borderstyle_option  { s!(:borderstyle, val[2])                        }
          | 'color'       '=' HEX_COLOR           { s!(:color, val[2])                              }
          | 'expand'      '=' NUMBER              { s!(:expand, Modifier::Expand.new(val[2].to_i))  }
          | 'expand'                              { s!(:expand, Modifier::Expand.new)               }
          | 'font'        '=' STRING              { s!(:fontfamily, val[2])                         }
          | 'fontcolor'   '=' HEX_COLOR           { s!(:fontcolor, val[2])                          }
          | 'fontfamily'  '=' STRING              { s!(:fontfamily, val[2])                         }
          | 'fontsize'    '=' NUMBER              { s!(:fontsize, val[2].to_f)                      }
          | 'format'      '=' format_options
          | 'freeze'                              { freeze!                                         }
          | 'note'        '=' STRING              { s!(:note, val[2])                               }
          | 'validate'    '=' condition           { s!(:validation, val[2])                         }

  format_options: format_options format_option | format_option { s!(:format, val[0]) }
  format_option: 'bold' | 'italic' | 'strikethrough' | 'underline'

  align_options: halign_option valign_option  { s!(:align, val[0]); s!(:align, val[1]) }
               | valign_option halign_option  { s!(:align, val[0]); s!(:align, val[1]) }
               | halign_option                { s!(:align, val[0]) }
               | valign_option                { s!(:align, val[0]) }

  halign_option: 'left' | 'center' | 'right'
  valign_option: 'top'  | 'center' | 'bottom'

  border_options: border_options border_option | border_option { s!(:border, val[0]) }
  border_option: 'all' | 'top' | 'right' | 'left' | 'bottom'

  borderstyle_option: 'dashed' | 'dotted' | 'double' | 'solid' | 'solid_medium' | 'solid_thick'

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
require 'strscan'
require_relative 'modifier'

---- inner
  attr_accessor :cell_modifier, :row_modifier

  def initialize
    @parsing_row = false
  end

  def parsing_row!
    @parsing_row = true
  end

  def finished_row!
    parsing_cell!
  end

  def parsing_cell!
    @parsing_row = false
    @cell_modifier.take_defaults_from!(@row_modifier)
  end

  def freeze!
    (@parsing_row ? @row_modifier : @cell_modifier).freeze!
  end

  def s!(property, value)
    target = @parsing_row ? @row_modifier : @cell_modifier
    target.public_send("#{property}=".to_sym, value)
  end

  def parse(text, cell_modifier:, row_modifier:, row_number: nil, cell_number: nil)
    if text.nil? || !(text.strip.start_with?("[[") || text.start_with?("![["))
      cell_modifier.take_defaults_from!(row_modifier)
      return text
    end

    @cell_modifier = cell_modifier
    @row_modifier = row_modifier

    tokens, value_without_modifier = [], ''
    s = StringScanner.new text
    until s.empty?
      case
      when s.scan(/\s+/)
      when s.scan(/\[\[/)
        tokens << [:START_CELL_MODIFIERS, s.matched]
      when s.scan(/\!\[\[/)
        tokens << [:START_ROW_MODIFIERS, s.matched]
      when s.scan(/\]\]/)
        # XXX we need to keep going if there are two modifiers
        tokens << [:END_MODIFIERS, s.matched]
        value_without_modifier = s.rest
        break
      when s.scan(/^#(([0-9a-fA-F]{2}){3}|([0-9a-fA-F]){3})/)
        tokens << [:HEX_COLOR, s.matched]
      when s.scan(/(['\w]+\!)?[\w\d]+:[\w\d]+/)
        tokens << [:A1_NOTATION, s.matched]
      when s.scan(/=/)
        tokens << [s.matched, s.matched]
      when s.scan(/-?[\d.]+/)
        tokens << [:NUMBER, s.matched]
      when s.scan(/'(?:[^'\\]|\\(?:['\\\/bfnrt]|u[0-9a-fA-F]{4}))*'/)
        tokens << [:STRING, s.matched.gsub(/^'|'$/, '')]
      when s.scan(/\//) 
        tokens << [:MODIFIER_SEPARATOR, s.matched]
      when s.scan(/\w+/)
        tokens << [s.matched, s.matched]
      else
        raise SyntaxError.new("Unable to parse starting at", s.peek(100),
                  row_number:, cell_number:,)
      end
    end

    define_singleton_method(:next_token) { tokens.shift }

    begin
      do_parse
    rescue Racc::ParseError => e
      raise SyntaxError.new("Error parsing modifier", e.message, 
                        wrapped_error: e, row_number:, cell_number:,)
    end

    value_without_modifier
  end

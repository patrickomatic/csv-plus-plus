class CSVPlusPlus::CodeSectionParser
prechigh
  left '(' ')'
  left '*' '/'
  left '+' '-'
  left '&'
preclow

token A1
      ASSIGN
      EOL 
      FALSE 
      ID 
      NUMBER 
      STRING 
      TRUE 
      VAR_EXPAND

rule
  code: code var | var

  var: ID ASSIGN exp                  { @variables[val[0]] = val[2] }

  exp: ID '(' fn_call_args ')'        { result = [[:fn, val[0]], val[2]]                }
     | ID '(' ')'                     { result = [[:fn, val[0]]]                        }
     | exp '&' exp                    { result = [[:fn, "CONCAT"], [val[0], val[2]]]    }
     | exp '*' exp                    { result = [[:fn, "MULTIPLY"], [val[0], val[2]]]  }
     | exp '/' exp                    { result = [[:fn, "DIVIDE"], [val[0], val[2]]]    }
     | exp '+' exp                    { result = [[:fn, "ADD"], [val[0], val[2]]]       }
     | exp '-' exp                    { result = [[:fn, "MINUS"], [val[0], val[2]]]     }
     | '(' exp ')'                    { result = [:group, [val[1]]]                     }
     | VAR_EXPAND ID                  { result = [:var, val[1]]                         } 
     | STRING                         { result = [:string, val[0].gsub('"', '')]        }
     | NUMBER                         { result = [:number, val[0].to_i]                 }
     | TRUE                           { result = [:boolean, true]                       }
     | FALSE                          { result = [:boolean, false]                      }
     | ID                             { result = [:id, val[0]]                          }

  fn_call_args: fn_call_args ',' exp  { result = [val[0], val[2]] }
              | exp                   { result = val[0]           }

end

---- header
require 'strscan'
require_relative 'syntax_error'
require_relative 'code_section'

---- inner
  attr_accessor :variables

  def parse(text)
    tokens = []

    s = StringScanner.new text
    until s.empty?
      case
      when s.scan(/\s+/)
      when s.scan(/\#[^\n]+\n/)
      when s.scan(/---/)
        break
      when s.scan(/\n/)
        tokens << [:EOL, s.matched]
      when s.scan(/:=/)
        tokens << [:ASSIGN, s.matched]
      when s.scan(/TRUE/)
        tokens << [:TRUE, s.matched]
      when s.scan(/FALSE/)
        tokens << [:FALSE, s.matched]
      when s.scan(/"(?:[^"\\]|\\(?:["\\\/bfnrt]|u[0-9a-fA-F]{4}))*"/)
        tokens << [:STRING, s.matched]
      when s.scan(/-?[\d.]+/)
        tokens << [:NUMBER, s.matched]
      when s.scan(/\$\$/)
        tokens << [:VAR_EXPAND, s.matched]
      when s.scan(/[\w_]+/)
        tokens << [:ID, s.matched]
      when s.scan(/[\(\)\{\}\/\*\+\-,=&]/)
        tokens << [s.matched, s.matched]
      else
        raise SyntaxError.new("Unable to parse starting at", s.rest)
      end
    end
    return CodeSection.new if tokens.empty?

    define_singleton_method(:next_token) { tokens.shift }

    @variables = {}
    begin
      do_parse
    rescue Racc::ParseError => e
      raise SyntaxError.new("Error parsing code section", e.message, wrapped_error: e)
    end
    CodeSection.new(@variables)
  end

class GSPush::CodeSectionParser
prechigh
  left '(' ')'
  left '*' '/'
  left '+' '-'
preclow
token ID 
      EOL
      NUMBER 
      STRING 
      TRUE
      FALSE
      ASSIGN
rule
  code: code var | var
 
  var: ID ASSIGN exp { @variables[val[0]]  = val[2] }
  
  exp: ID '(' fn_call_args ')'  { result = [val[0], val[2]]             }
     | ID '(' ')'               { result = [val[0]]                     }
     | exp '*' exp              { result = ["MULTIPLY", val[0], val[2]] }
     | exp '/' exp              { result = ["DIVIDE", val[0], val[2]]   }
     | exp '+' exp              { result = ["ADD", val[0], val[2]]      }
     | exp '-' exp              { result = ["MINUS", val[0], val[2]]    } 
     | '(' exp ')'              { result = [:group, val[1]]             }
     | literal                  { result = [:literal, val[0]]           }

  fn_call_args: fn_call_args ',' exp  { result = [val[0], val[2]] }
              | exp                   { result = val[0] }

  literal: STRING
         | NUMBER
         | TRUE
         | FALSE
         | ID
end

---- header
require 'strscan'

---- inner
  attr_accessor :variables

  def parse(text)
    tokens = []
    @variables = {}

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
      when s.scan(/[\$\w_]+/)
        tokens << [:ID, s.matched]
      when s.scan(/[\(\)\{\}\/\*\+\-,=]/) 
        tokens << [s.matched, s.matched]
      else
        raise "Unable to parse starting at: <#{s.rest}>"
      end 
    end
    return @variables if tokens.empty?

    define_singleton_method(:next_token) { tokens.shift }

    do_parse
 
    @variables
  end

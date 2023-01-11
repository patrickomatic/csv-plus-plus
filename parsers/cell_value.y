class CSVPlusPlus::Language::CellValueParser
prechigh
  left '(' ')'
  left '&'
  left '*' '/'
  left '+' '-'
preclow
token EOL
      FALSE
      ID
      NUMBER
      STRING
      TRUE
      VAR_REF

rule
  cell_value: '=' exp EOL                   { @ast = val[1]                                         }

  exp: ID '(' fn_call_args ')'              { result = Language::FunctionCall.new(val[0], val[2])   }
     | ID '(' ')'                           { result = Language::FunctionCall.new(val[0], [])       }
     | ID '(' exp ')'                       { result = Language::FunctionCall.new(val[0], [val[2]]) }
     | VAR_REF ID                           { result = Language::Variable.new(val[1])               }
     | STRING                               { result = Language::String.new(val[0].gsub('"', ''))   }
     | NUMBER                               { result = Language::Number.new(val[0])                 }
     | TRUE                                 { result = Language::Boolean.new(true)                  }
     | FALSE                                { result = Language::Boolean.new(false)                 }
     | ID                                   { result = val[0]                                       }

  fn_call_args: fn_call_args ',' exp        { result = [val[0], val[2]]                             }
              | exp                         { result = val[0]                                       }

end

---- header
require 'strscan'
require_relative 'syntax_error'

---- inner
  attr_accessor :ast

  def parse(text, execution_context)
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
      when s.scan(/\$\$/)
        tokens << [:VAR_REF, s.matched]
      when s.scan(/[\$\w_]+/)
        tokens << [:ID, s.matched]
      when s.scan(/[\(\)\/\*\+\-,=&]/)
        tokens << [s.matched, s.matched]
      else
        raise(
          SyntaxError.new(s.rest, execution_context),
          "Unable to parse starting at"
        )
      end 
    end
    tokens << [:EOL, :EOL]

    define_singleton_method(:next_token) { tokens.shift }

    begin
      do_parse
    rescue Racc::ParseError => e
      raise(
        SyntaxError.new(e.message, execution_context, wrapped_error: e), 
        "Error parsing code section"
      )
    end
    @ast
  end

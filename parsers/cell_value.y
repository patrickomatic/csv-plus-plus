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
  cell_value: '=' exp EOL                   { @ast = val[1]                                             }

  exp: ID '(' fn_call_args ')'              { result = entities_ns::FunctionCall.new(val[0], val[2])    }
     | ID '(' ')'                           { result = entities_ns::FunctionCall.new(val[0], [])        }
     | ID '(' exp ')'                       { result = entities_ns::FunctionCall.new(val[0], [val[2]])  }
     | VAR_REF ID                           { result = entities_ns::Variable.new(val[1])                }
     | STRING                               { result = entities_ns::String.new(val[0].gsub('"', ''))    }
     | NUMBER                               { result = entities_ns::Number.new(val[0])                  }
     | TRUE                                 { result = entities_ns::Boolean.new(true)                   }
     | FALSE                                { result = entities_ns::Boolean.new(false)                  }
     | ID                                   { result = entities_ns::CellReference.new(val[0])           }

  fn_call_args: fn_call_args ',' exp        { result = [val[0], val[2]]                                 }
              | exp                         { result = val[0]                                           }

end

---- header
  require_relative '../lexer'

---- inner
  include ::CSVPlusPlus::Lexer

  attr_accessor :ast

  def entities_ns
    ::CSVPlusPlus::Language::Entities
  end

  def tokenizer(scanner)
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /[\(\)\/\*\+\-,=&]/,
      ignore: /\s+/,
      scanner:,
      tokens: [
        [/true/i, :TRUE],
        [/false/i, :FALSE],
        [/"(?:[^"\\]|\\(?:["\\\/bfnrt]|u[0-9a-fA-F]{4}))*"/, :STRING],
        [/-?[\d.]+/, :NUMBER],
        [/\$\$/, :VAR_REF],
        [/[\$\w_]+/, :ID]
      ]
    )
  end

  def anything_to_parse?(input)
    input.strip.start_with?('=')
  end

  def return_value
    @ast
  end

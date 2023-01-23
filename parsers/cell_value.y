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
  cell_value: '=' exp EOL             { @ast = val[1]                               }

  exp: ID '(' fn_call_args ')'        { result = e(:function_call, val[0], val[2])  }
     | ID '(' ')'                     { result = e(:function_call, val[0], [])      }
     | VAR_REF ID                     { result = e(:variable, val[1])               }
     | STRING                         { result = e(:string, val[0])                 }
     | NUMBER                         { result = e(:number, val[0])                 }
     | TRUE                           { result = e(:boolean, true)                  }
     | FALSE                          { result = e(:boolean, false)                 }
     | ID                             { result = e(:cell_reference, val[0])         }

  fn_call_args: fn_call_args ',' exp  { result = val[0] << val[2]                   }
              | exp                   { result = [val[0]]                           }

end

---- header
  require_relative '../lexer'

---- inner
  include ::CSVPlusPlus::Lexer

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

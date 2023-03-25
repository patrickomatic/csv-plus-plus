class CSVPlusPlus::Parser::CellValue

prechigh
  left '(' ')'
  left '^'
  left '*' '/'
  left '+' '-'
  left '&'
  left '=' '<' '>' '<=' '>=' '<>'
preclow

token EOL
      FALSE
      ID
      INFIX_OP
      NUMBER
      STRING
      TRUE
      VAR_REF

rule
  cell_value: '=' exp EOL               { @ast = val[1]                                                 }

  exp: fn_call
     | infix_fn_call
     | '(' exp ')'                      { result = val[1] }
     | VAR_REF ID                       { result = variable(val[1])                                     }
     | STRING                           { result = string(val[0])                                       }
     | NUMBER                           { result = number(val[0])                                       }
     | TRUE                             { result = boolean(true)                                        }
     | FALSE                            { result = boolean(false)                                       }
     | ID                               { result = cell_reference(ref: val[0])                          }

  fn_call: ID '(' fn_call_args ')'      { result = function_call(val[0], val[2])                        }
         | ID '(' ')'                   { result = function_call(val[0], [])                            }

  fn_call_args: fn_call_args ',' exp    { result = val[0] << val[2]                                     }
              | exp                     { result = [val[0]]                                             }

  infix_fn_call: exp INFIX_OP exp       { result = function_call(val[1], [val[0], val[2]], infix: true) }

end

---- header
  require_relative '../lexer'
  require_relative '../entities/ast_builder'

---- inner
  include ::CSVPlusPlus::Entities::ASTBuilder
  include ::CSVPlusPlus::Lexer

  protected

  def anything_to_parse?(input)
    input.strip.start_with?('=')
  end

  def parse_subject
    'cell value'
  end

  def return_value
    @ast
  end

  def tokenizer
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /[\{\}\(\),=]/,
      ignore: /\s+/,
      tokens: [
        TOKEN_LIBRARY[:TRUE],
        TOKEN_LIBRARY[:FALSE],
        TOKEN_LIBRARY[:NUMBER],
        TOKEN_LIBRARY[:STRING],
        TOKEN_LIBRARY[:INFIX_OP],
        TOKEN_LIBRARY[:VAR_REF],
        TOKEN_LIBRARY[:ID]
      ]
    )
  end

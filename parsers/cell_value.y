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
      INFIX_OP
      NUMBER
      REF
      STRING
      TRUE
      VAR_REF

rule
  cell_value: '=' exp EOL             { @ast = val[1]                                                         }

  exp: fn_call
     | infix_fn_call
     | '(' exp ')'                    { result = val[1] }
     | STRING                         { result = string(val[0])                                               }
     | NUMBER                         { result = number(val[0])                                               }
     | TRUE                           { result = boolean(true)                                                }
     | FALSE                          { result = boolean(false)                                               }
     | REF                            { result = reference(ref: val[0])                                       }

  fn_call: REF '(' fn_call_args ')'   { result = function_call(val[0].to_sym, val[2])                         }
         | REF '(' ')'                { result = function_call(val[0].to_sym, [])                             }

  fn_call_args: fn_call_args ',' exp  { result = val[0] << val[2]                                             }
              | exp                   { result = [val[0]]                                                     }

  infix_fn_call: exp INFIX_OP exp     { result = function_call(val[1].to_sym, [val[0], val[2]], infix: true)  }

end

---- header
  require_relative '../lexer'
  require_relative '../lexer/racc_lexer'
  require_relative '../entities/ast_builder'

---- inner
  extend ::T::Sig
  extend ::T::Generic
  include ::CSVPlusPlus::Entities::ASTBuilder
  include ::CSVPlusPlus::Lexer::RaccLexer

  ReturnType = type_member {{ fixed: ::T.nilable(::CSVPlusPlus::Entities::Entity) }}

  protected

  sig { override.params(input: ::String).returns(::T::Boolean) }
  def anything_to_parse?(input)
    input.strip.start_with?('=')
  end

  sig { override.returns(::String) }
  def parse_subject
    'cell value'
  end

  sig { override.returns(ReturnType) }
  def return_value
    @ast
  end

  sig { override.returns(::CSVPlusPlus::Lexer::Tokenizer) }
  def tokenizer
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /[\{\}\(\),=]/,
      ignore: /\s+/,
      tokens: [
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:TRUE],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:FALSE],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:NUMBER],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:STRING],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:INFIX_OP],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:VAR_REF],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:REF]
      ]
    )
  end

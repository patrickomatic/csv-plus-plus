class CSVPlusPlus::Parser::CodeSection

prechigh
  right END_OF_CODE
  left '(' ')'
  left FN_DEF
  left ASSIGN
  left '^'
  left '*' '/'
  left '+' '-'
  left '&'
  left '=' '<' '>' '<=' '>=' '<>'
  left ','
preclow

token ASSIGN
      END_OF_CODE
      FALSE
      FN_DEF
      ID
      INFIX_OP
      NUMBER
      STRING
      TRUE
      VAR_REF

rule
  code_section: code END_OF_CODE | END_OF_CODE

  code: code def | def

  def: fn_def | var_def

  fn_def: FN_DEF ID fn_def_args_or_not exp    { def_function(val[1], val[2], val[3])                }

  fn_def_args_or_not: '(' fn_def_args ')'     { result = val[1]                                     }
                    | '(' ')'                 { result = []                                         }

  fn_def_args: fn_def_args ',' ID             { result = val[0] << val[2]                           }
             | ID                             { result = [val[0]]                                   }

  var_def: ID ASSIGN exp                      { def_variable(val[0], val[2])                        }

  exp: fn_call
     | infix_fn_call
     | '(' exp ')'                            { result = val[1] }
     | VAR_REF ID                             { result = variable(val[1])                           }
     | STRING                                 { result = string(val[0])                             }
     | NUMBER                                 { result = number(val[0])                             }
     | TRUE                                   { result = boolean(true)                              }
     | FALSE                                  { result = boolean(false)                             }
     | ID                                     { result = cell_reference(ref: val[0])                }
     
  infix_fn_call: exp INFIX_OP exp             { result = function_call(val[1], [val[0], val[2]], infix: true) }

  fn_call: ID '(' fn_call_args ')'            { result = function_call(val[0], val[2])              }
         | ID '(' ')'                         { result = function_call(val[0], [])                  }

  fn_call_args: fn_call_args ',' exp          { result = val[0] << val[2]                           }
              | exp                           { result = [val[0]]                                   }

end

---- header
  require_relative '../lexer'
  require_relative '../entities/ast_builder'

---- inner
  include ::CSVPlusPlus::Lexer
  include ::CSVPlusPlus::Entities::ASTBuilder

  def initialize(runtime)
    super()

    @runtime = runtime
  end

  protected

  def anything_to_parse?(input)
    @rest = input.strip

    return !@rest.index(::CSVPlusPlus::Lexer::END_OF_CODE_SECTION).nil?
  end

  def parse_subject
    'code section'
  end

  def tokenizer
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /[\{\}\(\),]/, # TODO: do I even need this (oh I think brackets are for arrays
      ignore: /\s+|\#[^\n]+\n/,
      stop_fn: lambda do |scanner|
        return false unless scanner.scan(/#{::CSVPlusPlus::Lexer::END_OF_CODE_SECTION}/)

        @tokens << [:END_OF_CODE, scanner.matched]
        @rest = scanner.rest.strip
        true
      end,
      tokens: [
        [/:=/, :ASSIGN],
        [/def/, :FN_DEF],
        TOKEN_LIBRARY[:TRUE],
        TOKEN_LIBRARY[:FALSE],
        TOKEN_LIBRARY[:NUMBER],
        TOKEN_LIBRARY[:STRING],
        TOKEN_LIBRARY[:INFIX_OP],
        TOKEN_LIBRARY[:VAR_REF],
        TOKEN_LIBRARY[:ID]
      ],
    )
  end

  def return_value
    @rest
  end

  private

  def def_function(id, arguments, body)
    fn_def = function(id, arguments, body)
    @runtime.def_function(fn_def.id, fn_def)
  end

  def def_variable(id, ast)
    @runtime.def_variable(id, ast)
  end

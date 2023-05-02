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
      INFIX_OP
      NUMBER
      REF
      STRING
      TRUE
      VAR_REF

rule
  code_section: code END_OF_CODE | END_OF_CODE

  code: code def | def

  def: fn_def | var_def

  fn_def: FN_DEF REF fn_def_args_or_not exp   { def_function(val[1].to_sym, val[2], val[3])                           }

  fn_def_args_or_not: '(' fn_def_args ')'     { result = val[1]                                                       }
                    | '(' ')'                 { result = []                                                           }

  fn_def_args: fn_def_args ',' REF            { result = val[0] << val[2]                                             }
             | REF                            { result = [val[0]]                                                     }

  var_def: REF ASSIGN exp                     { def_variable(val[0].to_sym, val[2])                                   }

  exp: fn_call
     | infix_fn_call
     | '(' exp ')'                            { result = val[1] }
     | STRING                                 { result = string(val[0])                                               }
     | NUMBER                                 { result = number(val[0])                                               }
     | TRUE                                   { result = boolean(true)                                                }
     | FALSE                                  { result = boolean(false)                                               }
     | REF                                    { result = reference(ref: val[0])                                       }
     
  infix_fn_call: exp INFIX_OP exp             { result = function_call(val[1].to_sym, [val[0], val[2]], infix: true)  }

  fn_call: REF '(' fn_call_args ')'           { result = function_call(val[0].to_sym, val[2])                         }
         | REF '(' ')'                        { result = function_call(val[0].to_sym, [])                             }

  fn_call_args: fn_call_args ',' exp          { result = val[0] << val[2]                                             }
              | exp                           { result = [val[0]]                                                     }

end

---- header
  require_relative '../lexer/racc_lexer'
  require_relative '../entities/ast_builder'

---- inner
  extend ::T::Sig
  extend ::T::Generic
  include ::CSVPlusPlus::Lexer::RaccLexer
  include ::CSVPlusPlus::Entities::ASTBuilder

  ReturnType = type_member {{ fixed: ::T.nilable(::String) }}

  sig { params(scope: ::CSVPlusPlus::Runtime::Scope).void }
  def initialize(scope)
    super()
    @scope = scope
  end

  protected

  sig { override.params(input: ::String).returns(::T::Boolean) }
  def anything_to_parse?(input)
    @rest = input.strip

    return !@rest.index(::CSVPlusPlus::Lexer::END_OF_CODE_SECTION).nil?
  end

  sig { override.returns(::String) }
  def parse_subject
    'code section'
  end

  sig { override.returns(::CSVPlusPlus::Lexer::Tokenizer) }
  def tokenizer
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /[\{\}\(\),]/, # TODO: do I even need this (oh I think brackets are for arrays
      ignore: /\s+|\#.*/,
      stop_fn: lambda do |scanner|
        return false unless scanner.scan(/#{::CSVPlusPlus::Lexer::END_OF_CODE_SECTION}/)

        @tokens << [:END_OF_CODE, scanner.matched]
        @rest = scanner.rest.strip
        true
      end,
      tokens: [
        ::CSVPlusPlus::Lexer::Token.new(regexp: /:=/, token: :ASSIGN),
        ::CSVPlusPlus::Lexer::Token.new(regexp: /\bdef\b/, token: :FN_DEF),
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:TRUE],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:FALSE],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:NUMBER],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:STRING],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:INFIX_OP],
        ::CSVPlusPlus::Lexer::TOKEN_LIBRARY[:REF]
      ],
    )
  end

  sig { override.returns(ReturnType) }
  def return_value
    @rest
  end

  private

  sig do
    params(id: ::Symbol, arguments: ::T::Array[::Symbol], body: ::CSVPlusPlus::Entities::Entity)
      .returns(::CSVPlusPlus::Entities::Entity)
  end
  def def_function(id, arguments, body)
    @scope.def_function(id, function(id, arguments, body))
  end

  sig do
    params(id: ::Symbol, ast: ::CSVPlusPlus::Entities::Entity)
      .returns(::CSVPlusPlus::Entities::Entity)
  end
  def def_variable(id, ast)
    @scope.def_variable(id, ast)
  end

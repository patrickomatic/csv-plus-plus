class CSVPlusPlus::Language::CodeSectionParser

prechigh
  right END_OF_CODE
  left '(' ')'
  left FN_DEF
  left ASSIGN
  left ','
preclow

token ASSIGN
      CELL_REF
      END_OF_CODE
      EOL
      FALSE
      FN_DEF
      ID
      NUMBER
      STRING
      TRUE
      VAR_REF

rule
  code_section: code END_OF_CODE | END_OF_CODE

  code: code def | def

  def: fn_def | var_def

  fn_def: FN_DEF ID '(' fn_def_args ')' exp   { def_function(val[1], val[3], val[5])                }
  fn_def: FN_DEF ID '(' ')' exp               { def_function(val[1], [], val[4])                    }

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
     | ID                                     { result = cell_reference(val[0])                     }
     
  infix_fn_call: exp '&' exp                  { result = function_call(:concat, [val[0], val[2]])   }
               | exp '*' exp                  { result = function_call(:multiply, [val[0], val[2]]) }
               | exp '+' exp                  { result = function_call(:add, [val[0], val[2]])      }
               | exp '-' exp                  { result = function_call(:minus, [val[0], val[2]])    }
               | exp '/' exp                  { result = function_call(:divide, [val[0], val[2]])   }

  fn_call: ID '(' fn_call_args ')'            { result = function_call(val[0], val[2])              }
         | ID '(' ')'                         { result = function_call(val[0], [])                  }

  fn_call_args: fn_call_args ',' exp          { result = val[0] << val[2]                           }
              | exp                           { result = [val[0]]                                   }

end

---- header
  require_relative '../lexer'
  require_relative '../code_section'
  require_relative '../language/ast_builder'

---- inner
  include ::CSVPlusPlus::Lexer
  include ::CSVPlusPlus::Language::ASTBuilder

  def initialize
    super
    @code_section = CodeSection.new
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
      catchall: /[\(\)\{\}\/\*\+\-,=&]/,
      ignore: /\s+|\#[^\n]+\n/,
      stop_fn: lambda do |scanner|
        return false unless scanner.scan(/#{::CSVPlusPlus::Lexer::END_OF_CODE_SECTION}/)

        @tokens << [:END_OF_CODE, scanner.matched]
        @rest = scanner.rest.strip
        true
      end,
      tokens: [
        [/\n/, :EOL],
        [/:=/, :ASSIGN],
        [/\bdef\b/, :FN_DEF],
        [/\bTRUE\b/i, :TRUE],
        [/\bFALSE\b/i, :FALSE],
        [/"(?:[^"\\]|\\(?:["\\\/bfnrt]|u[0-9a-fA-F]{4}))*"/, :STRING],
        [/-?[\d.]+/, :NUMBER],
        [/\$\$/, :VAR_REF],
        [/[!:\w_]+/, :ID],
      ],
    )
  end

  def return_value
    [@code_section, @rest]
  end

  private

  def def_function(id, arguments, body)
    fn_def = function(id, arguments, body)
    @code_section.def_function(fn_def.id, fn_def)
  end

  def def_variable(id, ast)
    @code_section.def_variable(id, ast)
  end

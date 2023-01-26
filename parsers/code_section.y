class CSVPlusPlus::Language::CodeSectionParser

prechigh
  left FN_DEF
  left ASSIGN
  left '(' ')'
  left ','
  right END_OF_CODE
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

  fn_def: FN_DEF ID '(' fn_def_args ')' exp   { def_function(val[1], val[3], val[5])          }
  fn_def: FN_DEF ID '(' ')' exp               { def_function(val[1], [], val[4])              }

  fn_def_args: fn_def_args ',' ID             { result = val[0] << val[2]                     }
             | ID                             { result = [val[0]]                             }

  var_def: ID ASSIGN exp                      { def_variable(val[0], val[2])                  }

  exp: ID '(' fn_call_args ')'                { result = e(:function_call, val[0], val[2])    }
     | ID '(' ')'                             { result = e(:function_call, val[0], [])        }
     | ID '(' exp ')'                         { result = e(:function_call, val[0], [val[2]])  }
     | VAR_REF ID                             { result = e(:variable, val[1])                 }
     | STRING                                 { result = e(:string, val[0])                   }
     | NUMBER                                 { result = e(:number, val[0])                   }
     | TRUE                                   { result = e(:boolean, true)                    }
     | FALSE                                  { result = e(:boolean, false)                   }
     | ID                                     { result = e(:cell_reference, val[0])           }

  fn_call_args: fn_call_args ',' exp          { result = val[0] << val[2]                     }
              | exp                           { result = [val[0]]                             }

end

---- header
require_relative '../lexer'
require_relative '../code_section'

---- inner
  include ::CSVPlusPlus::Lexer

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

  def tokenizer(input)
    ::CSVPlusPlus::Lexer::Tokenizer.new(
      catchall: /[\(\)\{\}\/\*\+\-,=&]/, # TODO this might not even be used
      ignore: /\s+|\#[^\n]+\n/,
      input:,
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
        [/\bTRUE\b/, :TRUE],
        [/\bFALSE\b/, :FALSE],
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

  def e(type, *entity_args)
    ::CSVPlusPlus::Language::TYPES[type].new(*entity_args)
  end

  def def_function(id, arguments, body)
    fn_def = e(:function, id, arguments, body)
    @code_section.def_function(fn_def.id, fn_def)
  end

  def def_variable(id, ast)
    @code_section.def_variable(id, ast)
  end

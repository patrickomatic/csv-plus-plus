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
require 'strscan'
require_relative '../code_section'
require_relative 'entities'

---- inner
  def e(type, *entity_args)
    ::CSVPlusPlus::Language::TYPES[type].new(*entity_args)
  end

  def def_function(id, arguments, body)
    fn_def = ::CSVPlusPlus::Language::Entities::Function.new(id, arguments, body)
    @code_section.def_function(fn_def.id, fn_def)
  end

  def def_variable(id, ast)
    @code_section.def_variable(id, ast)
  end

  def parse(input, runtime)
    text = input.read.strip
    @code_section = CodeSection.new

    eoc = ::CSVPlusPlus::Lexer::END_OF_CODE_SECTION
    eoc_index = text.index(eoc)
    return @code_section, text if eoc_index.nil?

    tokens, rest = [], ''

    s = StringScanner.new(text)
    until s.empty?
      case
      when s.scan(/\s+/)
      when s.scan(/\#[^\n]+\n/)
      when s.scan(/#{eoc}/)
        tokens << [:END_OF_CODE, s.matched]
        rest = s.rest.strip
        break
      when s.scan(/\n/)
        tokens << [:EOL, s.matched]
      when s.scan(/:=/)
        tokens << [:ASSIGN, s.matched]
      when s.scan(/\bdef\b/)
        tokens << [:FN_DEF, s.matched]
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
      when s.scan(/[!:\w_]+/)
        tokens << [:ID, s.matched]
      when s.scan(/[\(\)\{\}\/\*\+\-,=&]/) # XXX I don't think this is used, get rid of this
        tokens << [s.matched, s.matched]
      else
        runtime.raise_syntax_error('Unable to parse code section starting at', s.peek(100))
      end
    end

    return @code_section, rest if tokens.empty?

    define_singleton_method(:next_token) { tokens.shift }

    begin
      do_parse
    rescue Racc::ParseError => e
      runtime.raise_syntax_error('Error parsing code section', e.message, wrapped_error: e)
    end

    return @code_section, rest
  end

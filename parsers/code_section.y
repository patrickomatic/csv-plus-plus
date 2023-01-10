class CSVPlusPlus::Language::CodeSectionParser

prechigh
  left FN_DEF
  left ASSIGN
  left '(' ')'
  left ','
preclow

token ASSIGN
      EOL
      FALSE
      FN_DEF
      ID
      NUMBER
      STRING
      TRUE
      VAR_REF

rule
  code: code def | def

  def: fn_def | var_def

  fn_def: FN_DEF ID '(' fn_def_args ')' exp   { @code_section.def_function(val[0], val[2], val[3])    }
  fn_def: FN_DEF ID '(' ')' exp               { @code_section.def_function(val[0], [], val[3])        }

  fn_def_args: fn_def_args ',' ID             { result = [val[0], val[2]]                             }
             | ID                             { result = val[0]                                       }

  var_def: ID ASSIGN exp                      { @code_section.def_variable(val[0], val[2])            }

  exp: ID '(' fn_call_args ')'                { result = Language::FunctionCall.new(val[0], val[2])   }
     | ID '(' ')'                             { result = Language::FunctionCall.new(val[0], [])       }
     | ID '(' exp ')'                         { result = Language::FunctionCall.new(val[0], [val[2]]) }
     | VAR_REF ID                             { result = Language::Variable.new(val[1])               }
     | STRING                                 { result = Language::String.new(val[0])                 }
     | NUMBER                                 { result = Language::Number.new(val[0])                 }
     | TRUE                                   { result = Language::Boolean.new(true)                  }
     | FALSE                                  { result = Language::Boolean.new(false)                 }
     | ID                                     { result = val[0]                                       }

  fn_call_args: fn_call_args ',' exp          { result = [val[0], val[2]]                             }
              | exp                           { result = val[0]                                       }

end

---- header
require 'strscan'
require_relative './global_scope'
require_relative './syntax_error'
require_relative '../code_section'

---- inner
  def parse(execution_context)
    rest = nil
    execution_context.parsing_code_section! do |input|
      text = input.read
      @code_section = CodeSection.new

      eoc_index = text.index(Language::END_OF_CODE_SECTION)
      next text if eoc_index.nil?

      tokens = []

      s = StringScanner.new text
      until s.empty?
        case
        when s.scan(/\s+/)
        when s.scan(/\#[^\n]+\n/)
        when s.scan(/---/)
          break
        when s.scan(/\n/)
          tokens << [:EOL, s.matched]
        when s.scan(/:=/)
          tokens << [:ASSIGN, s.matched]
        when s.scan(/\bdef\b/)
          tokens << [:FUNCTION_DEF, s.matched]
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
        when s.scan(/[\w_]+/)
          tokens << [:ID, s.matched]
        when s.scan(/[\(\)\{\}\/\*\+\-,=&]/)
          tokens << [s.matched, s.matched]
        else
          raise SyntaxError.new("Unable to parse starting at", s.rest, execution_context)
        end
      end

      next text if tokens.empty?

      define_singleton_method(:next_token) { tokens.shift }

      begin
        do_parse
      rescue Racc::ParseError => e
        raise SyntaxError.new("Error parsing code section", e.message, execution_context,
                              wrapped_error: e)
      end

      # return the rest of the file (the CSV part) to the execution_context because they're
      # going to use it to rewrite the input file and further parse
      s.rest
    end

    @code_section
  end

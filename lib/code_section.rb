require_relative 'function'
require_relative 'syntax_error'
require_relative 'code_section_parser.tab'

module GSPush
  class CodeSection
    END_OF_CODE_SECTION = "---"

    attr_reader :variables, :functions

    def initialize(variables = {})
      @variables = variables
    end

    def self.parse!(input)
      all_lines = input.readlines.map(&:strip)
      unless all_lines.include? END_OF_CODE_SECTION
        # no code section, just rewind our read and leave it alone
        input.rewind
        return CodeSection.new
      end

      variables = CodeSectionParser.new.parse(input.read)
      input.rewind

      csv_lines, in_code_section = [], true
      all_lines.each_with_index do |line|
        csv_lines << line
        break if line == END_OF_CODE_SECTION
      end

      input.write(csv_lines)
      input.rewind

      CodeSection.new(variables)

=begin
      variables, functions = {}, {}
        line.gsub!(/\#.*/, '')
        line.strip!
        next if line.empty?

        if line == END_OF_CODE_SECTION
          if parsing_function
            raise SyntaxError.new("Unterminated function #{parsing_function.name}", line, 
                                  line_number: parsing_function.line_number)
          end
          in_code_section = false 
          next
        end

        if parsing_function
          parsing_function.body = parsing_function.body + line
          if line.include? '}'
            functions[parsing_function.name] = parsing_function
            parsing_function = nil
          end
        end

        case line
        when /(\w+)\s+:=\s+(.+)$/
          variables[$1] = $2
        when /\s*=\s*(\w+)\(([\w\s,]*)\)\s*\{([^}]*)\s*\}?/
          f = Function.new($1, $2.strip, $3.strip, line_number)
          if line.include? '}'
            # it's all on one line
            functions[f.name] = f
          else
            # it spans multiple lines, we gotta keep parsing until we get a }
            parsing_function = f
          end
        else
          if parsing_function
            raise SyntaxError.new("Unterminated function #{parsing_function_name}", line, 
                                  line_number: parsing_function.line_number)
          else
            raise SyntaxError.new('Invalid line', line, line_number:)
          end
        end
      end

      input.write csv_lines
      input.rewind
      CodeSection.new(variables:, functions:)
=end
    end
  end
end

require_relative 'ast'
require_relative 'function'
require_relative 'syntax_error'
require_relative 'code_section.tab'

module CSVPlusPlus
  class CodeSection
    attr_reader :variables

    def initialize(variables = {})
      @variables = variables
    end

    def self.parse!(input)
      all_lines = input.readlines.map(&:strip)
      input.rewind

      eoc_index = all_lines.index(AST::END_OF_CODE_SECTION)
      return CodeSection.new if eoc_index.nil?

      code_section = CodeSectionParser.new.parse(all_lines.join("\n"))

      csv_lines = all_lines[(eoc_index + 1) ...]

      input.truncate(0)
      input.write(csv_lines.join("\n"))
      input.rewind

      code_section
    end
  end
end

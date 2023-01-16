# frozen_string_literal: true

require_relative 'language/compiler'
require_relative 'writer/writer'

# A language for writing rich CSV data
module CSVPlusPlus
  # Create a template and output it using a GoogleSheet
  def self.apply_template_to_sheet!(input, filename, options)
    ::CSVPlusPlus::Language::Compiler.with_compiler(input:, filename:, options:) do |c|
      template = c.parse_template

      output = ::CSVPlusPlus::Writer.writer(options)
      c.outputting! { output.write(template) }
    end
  end
end

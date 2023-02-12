# frozen_string_literal: true

require_relative 'csv_plus_plus/error'
require_relative 'csv_plus_plus/language/compiler'
require_relative 'csv_plus_plus/options'
require_relative 'csv_plus_plus/writer'

# A language for writing rich CSV data
module CSVPlusPlus
  # Parse the input into a +Template+ and write it to the desired format
  def self.apply_template_to_sheet!(input, filename, options)
    warn(options.verbose_summary) if options.verbose

    ::CSVPlusPlus::Language::Compiler.with_compiler(input:, filename:, options:) do |c|
      template = c.parse_template

      output = ::CSVPlusPlus::Writer.writer(options)
      c.outputting! { output.write(template) }
    end
  end
end

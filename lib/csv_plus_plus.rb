# frozen_string_literal: true

require 'google/apis/drive_v3'
require 'google/apis/sheets_v4'
require 'googleauth'
require 'rubyXL'
require 'rubyXL/convenience_methods'

require_relative 'csv_plus_plus/cli'
require_relative 'csv_plus_plus/error'
require_relative 'csv_plus_plus/language/builtins'
require_relative 'csv_plus_plus/language/compiler'
require_relative 'csv_plus_plus/language/runtime'
require_relative 'csv_plus_plus/options'
require_relative 'csv_plus_plus/writer'

# A programming language for writing rich CSV files
module CSVPlusPlus
  # Parse the input into a +Template+ and write it to the desired format
  #
  # @param input [String] The csvpp input to compile
  # @param filename [String, nil] The filename the input was read from.  +nil+ if it is read from stdin.
  # @param options [Options] The various options to compile with
  #
  # rubocop:disable Metrics/MethodLength
  def self.apply_template_to_sheet!(input, filename, options)
    warn(options.verbose_summary) if options.verbose

    ::CSVPlusPlus::Language::Compiler.with_compiler(
      options:,
      runtime: ::CSVPlusPlus::Language::Runtime.new(input:, filename:)
    ) do |c|
      template = c.compile_template

      output = ::CSVPlusPlus::Writer.writer(options)
      c.outputting! do
        output.write_backup if options.backup
        output.write(template)
      end
    end
  end
  # rubocop:enable Metrics/MethodLength
end

# frozen_string_literal: true

require 'google/apis/drive_v3'
require 'google/apis/sheets_v4'
require 'googleauth'
require 'rubyXL'
require 'rubyXL/convenience_methods'
require 'set'

require_relative 'csv_plus_plus/cell'
require_relative 'csv_plus_plus/cli'
require_relative 'csv_plus_plus/code_section'
require_relative 'csv_plus_plus/color'
require_relative 'csv_plus_plus/error'
require_relative 'csv_plus_plus/language/builtins'
require_relative 'csv_plus_plus/language/compiler'
require_relative 'csv_plus_plus/language/runtime'
require_relative 'csv_plus_plus/language/syntax_error'
require_relative 'csv_plus_plus/modifier'
require_relative 'csv_plus_plus/modifier.tab'
require_relative 'csv_plus_plus/options'
require_relative 'csv_plus_plus/row'
require_relative 'csv_plus_plus/template'
require_relative 'csv_plus_plus/writer'

# A programming language for writing rich CSV files
module CSVPlusPlus
  # Parse the input into a +Template+ and write it to the desired format
  #
  # @param input [String] The csvpp input to compile
  # @param filename [String, nil] The filename the input was read from.  +nil+ if it is read from stdin.
  # @param options [Options] The various options to compile with
  def self.apply_template_to_sheet!(input, filename, options)
    warn(options.verbose_summary) if options.verbose

    runtime = ::CSVPlusPlus::Language::Runtime.new(input:, filename:)

    ::CSVPlusPlus::Language::Compiler.with_compiler(options:, runtime:) do |compiler|
      template = compiler.compile_template

      warn(template.verbose_summary) if options.verbose

      write_template(template, compiler, options)
    end
  end

  # Write the results (and possibly make a backup) of a compiled +template+
  #
  # @param template [Template] The compiled template
  # @param compiler [Compiler] The compiler currently in use
  # @param options [Options] The options we're running with
  def self.write_template(template, compiler, options)
    compiler.outputting! do
      output = ::CSVPlusPlus::Writer.writer(options)
      output.write_backup if options.backup
      output.write(template)
    end
  end
end

# typed: strict
# frozen_string_literal: true

require 'sorbet-runtime'

require 'benchmark'
require 'csv'
require 'fileutils'
require 'google/apis/drive_v3'
require 'google/apis/sheets_v4'
require 'googleauth'
require 'optparse'
require 'pathname'
require 'rubyXL'
require 'rubyXL/convenience_methods'
require 'set'
require 'tempfile'

require_relative 'csv_plus_plus/source_code'

require_relative 'csv_plus_plus/runtime'

require_relative 'csv_plus_plus/cli_flag'
require_relative 'csv_plus_plus/entities'
require_relative 'csv_plus_plus/error'

require_relative 'csv_plus_plus/cell'
require_relative 'csv_plus_plus/cli'
require_relative 'csv_plus_plus/color'
require_relative 'csv_plus_plus/modifier'

require_relative 'csv_plus_plus/parser/cell_value.tab'
require_relative 'csv_plus_plus/parser/code_section.tab'
require_relative 'csv_plus_plus/parser/modifier.tab'

require_relative 'csv_plus_plus/compiler'

require_relative 'csv_plus_plus/google_options'
require_relative 'csv_plus_plus/lexer'
require_relative 'csv_plus_plus/options'
require_relative 'csv_plus_plus/row'
require_relative 'csv_plus_plus/template'
require_relative 'csv_plus_plus/writer'

require_relative 'csv_plus_plus/benchmarked_compiler'

# A programming language for writing rich CSV files
module CSVPlusPlus
  extend ::T::Sig

  sig { params(input: ::String, filename: ::T.nilable(::String), options: ::CSVPlusPlus::Options).void }
  # Parse the input into a +Template+ and write it to the desired format
  #
  # @param input [String] The csvpp input to compile
  # @param filename [String, nil] The filename the input was read from.  +nil+ if it is read from stdin.
  # @param options [Options] The various options to compile with
  def self.apply_template_to_sheet!(input, filename, options)
    warn(options.verbose_summary) if options.verbose

    runtime = ::CSVPlusPlus::Runtime.new(source_code: ::CSVPlusPlus::SourceCode.new(input:, filename:))

    ::CSVPlusPlus::Compiler.with_compiler(options:, runtime:) do |compiler|
      template = compiler.compile_template
      warn(template.verbose_summary) if options.verbose

      write_template(template:, compiler:, options:)
    end
  end

  sig do
    params(compiler: ::CSVPlusPlus::Compiler, options: ::CSVPlusPlus::Options, template: ::CSVPlusPlus::Template).void
  end
  # Write the results (and possibly make a backup) of a compiled +template+
  #
  # @param compiler [Compiler] The compiler currently in use
  # @param options [Options] The options we're running with
  # @param template [Template] The compiled template
  def self.write_template(compiler:, options:, template:)
    compiler.outputting! do |runtime|
      output = ::CSVPlusPlus::Writer.writer(options, runtime)
      output.write_backup if options.backup
      output.write(template)
    end
  end
end

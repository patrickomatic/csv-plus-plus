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
require 'strscan'
require 'tempfile'

require_relative 'csv_plus_plus/a1_reference'
require_relative 'csv_plus_plus/google_api_client'
require_relative 'csv_plus_plus/options'
require_relative 'csv_plus_plus/runtime/position'
require_relative 'csv_plus_plus/source_code'

require_relative 'csv_plus_plus/cli_flag'
require_relative 'csv_plus_plus/entities'
require_relative 'csv_plus_plus/error'
require_relative 'csv_plus_plus/error_formatter'

require_relative 'csv_plus_plus/cli'
require_relative 'csv_plus_plus/runtime'

require_relative 'csv_plus_plus/cell'
require_relative 'csv_plus_plus/color'
require_relative 'csv_plus_plus/modifier'

require_relative 'csv_plus_plus/parser/cell_value.tab'
require_relative 'csv_plus_plus/parser/code_section.tab'
require_relative 'csv_plus_plus/parser/modifier.tab'

require_relative 'csv_plus_plus/compiler'

require_relative 'csv_plus_plus/lexer'
require_relative 'csv_plus_plus/reader'
require_relative 'csv_plus_plus/row'
require_relative 'csv_plus_plus/template'
require_relative 'csv_plus_plus/writer'

require_relative 'csv_plus_plus/benchmarked_compiler'

# A programming language for writing rich CSV files
module CSVPlusPlus
  extend ::T::Sig

  sig { params(source_code: ::CSVPlusPlus::SourceCode, options: ::CSVPlusPlus::Options::Options).void }
  # Parse the input into a +Template+ and write it to the desired format
  #
  # @param source_code [SourceCode] The source code being compiled
  # @param options [Options] The various options to compile with
  def self.cli_compile(source_code, options)
    runtime = ::CSVPlusPlus::Runtime.new(source_code:)

    warn(options.verbose_summary) if options.verbose

    ::CSVPlusPlus::Compiler.with_compiler(options:, runtime:) do |compiler|
      template = compiler.compile_template
      warn(template.verbose_summary) if options.verbose

      write_template(template:, compiler:, options:)
    end
  rescue ::StandardError => e
    ::CSVPlusPlus::ErrorFormatter.new(runtime: ::T.must(runtime), options:).handle_error(e)
    # the caller will exit(1)
    raise(e)
  end

  sig do
    params(
      compiler: ::CSVPlusPlus::Compiler,
      options: ::CSVPlusPlus::Options::Options,
      template: ::CSVPlusPlus::Template
    ).void
  end
  # Write the results (and possibly make a backup) of a compiled +template+
  #
  # @param compiler [Compiler] The compiler currently in use
  # @param options [Options] The options we're running with
  # @param template [Template] The compiled template
  def self.write_template(compiler:, options:, template:)
    compiler.outputting! do |position|
      output = ::CSVPlusPlus::Writer.writer(options, position)
      output.write_backup if options.backup
      output.write(template)
    end
  end
end

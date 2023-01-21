# frozen_string_literal: true

require 'benchmark'
require 'csv'
require_relative '../cell'
require_relative '../modifier'
require_relative '../modifier.tab'
require_relative '../row'
require_relative '../template'
require_relative 'code_section.tab'
require_relative 'entities'
require_relative 'runtime'
require_relative 'scope'

module CSVPlusPlus
  module Language
    ##
    # Encapsulates the parsing and building of objects (+Template+ -> +Row+ -> +Cell+).
    # Variable resolution is delegated to the +Scope+
    # rubocop:disable Metrics/ClassLength
    class Compiler
      attr_reader :timings, :benchmark, :options, :runtime, :scope

      # Create a compiler and make sure it gets cleaned up
      def self.with_compiler(input:, filename:, options:, &block)
        runtime = ::CSVPlusPlus::Language::Runtime.new(filename:, input:)

        if options.verbose
          compiler_with_timings(runtime:, options:) do |c|
            block.call(c)
          end
        else
          yield(new(runtime:, options:))
        end
      ensure
        runtime.cleanup!
      end

      # Create a compiler that can time each of it's stages
      def self.compiler_with_timings(options:, runtime:, &block)
        ::Benchmark.benchmark(::Benchmark::CAPTION, 25, ::Benchmark::FORMAT, '> Total') do |x|
          compiler = new(options:, runtime:, benchmark: x)
          block.call(compiler)
          [compiler.timings.reduce(:+)]
        end
      end

      # initialize
      def initialize(runtime:, options:, scope: nil, benchmark: nil)
        @options = options
        @runtime = runtime
        @scope = scope || ::CSVPlusPlus::Language::Scope.new(runtime:)
        @benchmark = benchmark
        @timings = [] if benchmark
      end

      # Parse an entire template and return a +::CSVPlusPlus::Template+ instance
      def parse_template
        parse_code_section!
        rows = parse_csv_section!

        ::CSVPlusPlus::Template.new(rows:, scope: @scope).tap do |t|
          t.validate_infinite_expands(@runtime)
          expanding { t.expand_rows! }
          resolve_all_cells!(t)
        end
      end

      # parses the input file and returns a +CodeSection+
      def parse_code_section!
        parsing_code_section do |input|
          code_section, csv_section = ::CSVPlusPlus::Language::CodeSectionParser.new.parse(input, self)
          # TODO: infer a type
          # allow user-supplied key/values to override anything global or from the code section
          code_section.def_variables(
            options.key_values.transform_values { |v| ::CSVPlusPlus::Language::Entities::String.new(v.to_s) }
          )
          @scope.code_section = code_section

          # return the csv_section to the caller because they're gonna re-write input with it
          next csv_section
        end
        @scope.code_section
      end

      # workflow when parsing csv
      def parse_csv_section!
        workflow(stage: 'Parsing CSV section') do
          @runtime.map_rows(::CSV.new(runtime.input)) do |csv_row|
            parse_row(csv_row)
          end
        end
      ensure
        # we're done with the file and everything is in memory
        @runtime.cleanup!
      end

      # Using the current +@runtime+ and the given +csv_row+ parse it into a +Row+ of +Cell+s
      # +csv_row+ should have already been run through a CSV parser and is an array of strings
      def parse_row(csv_row)
        row_modifier = ::CSVPlusPlus::Modifier.new(row_level: true)

        cells =
          @runtime.map_row(csv_row) do |value, _cell_index|
            cell_modifier = ::CSVPlusPlus::Modifier.new
            parsed_value = ::CSVPlusPlus::ModifierParser.new.parse(
              value, runtime: @runtime, row_modifier:, cell_modifier:
            )

            ::CSVPlusPlus::Cell.parse(parsed_value, runtime:, modifier: cell_modifier)
          end

        ::CSVPlusPlus::Row.new(@runtime.row_index, cells, row_modifier)
      end

      # workflow when resolving the values of all cells
      def resolve_all_cells!(template)
        workflow(stage: 'Resolving each cell') do
          @runtime.map_rows(template.rows, cells_too: true) do |cell|
            cell.ast = @scope.resolve_cell_value if cell.ast
          end
        end
      end

      # workflow when writing results
      def outputting!(&block)
        workflow(stage: 'Writing the spreadsheet') do
          block.call
        end
      end

      # to_s
      def to_s
        "Compiler(options: #{@options}, runtime: #{@runtime}, scope: #{@scope})"
      end

      private

      # workflow when parsing the code section
      def parsing_code_section(&block)
        workflow(
          stage: 'Parsing code section',
          processing_code_section: true
        ) do
          csv_section = block.call(@runtime.input)
          @runtime.rewrite_input!(csv_section)
        end
      end

      # workflow when expanding rows
      def expanding(&block)
        workflow(stage: 'Expanding rows') do
          block.call
        end
      end

      def workflow(stage:, processing_code_section: false, &block)
        @runtime.init!(processing_code_section ? 1 : (@runtime.length_of_code_section || 1))

        ret = nil
        if @benchmark
          @timings << @benchmark.report(stage) { ret = block.call }
        else
          ret = block.call
        end

        ret
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end

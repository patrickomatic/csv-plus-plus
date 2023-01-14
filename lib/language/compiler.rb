# frozen_string_literal: true

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
      attr_reader :scope, :options, :runtime

      # Create a compiler and make sure it gets cleaned up
      def self.with_compiler(input:, filename:, options:)
        runtime = ::CSVPlusPlus::Language::Runtime.new(filename:, input:)
        yield(new(runtime:, options:))
      ensure
        runtime.cleanup!
      end

      # initialize
      def initialize(runtime:, options:, scope: nil)
        @options = options
        @runtime = runtime
        @scope = scope || ::CSVPlusPlus::Language::Scope.new(runtime:)
      end

      # Parse an entire template and return a +::CSVPlusPlus::Template+ instance
      def parse_template
        parse_code_section!
        rows = parse_csv_section!

        # TODO: should probably just flip this so it goes Template -> Scope -> CodeSection
        ::CSVPlusPlus::Template.new(rows:, scope: @scope).tap do |t|
          t.validate_infinite_expands(@runtime)
          expanding { t.expand_rows! }
          # TODO: wrap these in a workflow (I guess?)
          resolve_all_cells!(t)
          apply_all_functions!(t)
        end
      end

      # parses the input file and returns a +CodeSection+
      def parse_code_section!
        parsing_code_section do |input|
          code_section, csv_section = ::CSVPlusPlus::Language::CodeSectionParser.new.parse(input, self)
          # TODO: infer a type
          # allow user-supplied key/values to override anything global or from the code section
          code_section.def_variables(
            options.key_values.transform_values { |v| ::CSVPlusPlus::Language::String.new(v.to_s) }
          )
          @scope.code_section = code_section

          # return the csv_section to the caller because they're gonna re-write input with it
          next csv_section
        end
        @scope.code_section
      end

      # workflow when parsing csv
      def parse_csv_section!
        rows = nil
        workflow(log_subject: 'parsing CSV section') do
          rows =
            @runtime.map_rows(::CSV.new(runtime.input)) do |csv_row|
              parse_row(csv_row)
            end
        end

        # we're done with the file and everything is in memory
        @runtime.cleanup!

        rows
      end

      # Using the current +@runtime+ and the given +csv_row+ parse it into a +Row+ of +Cell+s
      # +csv_row+ should have already been run through a CSV parser and is an array of strings
      def parse_row(csv_row)
        row_modifier = ::CSVPlusPlus::Modifier.new(row_level: true)

        cells =
          @runtime.map_row(csv_row) do |value, cell_index|
            cell_modifier = ::CSVPlusPlus::Modifier.new
            parsed_value = ::CSVPlusPlus::ModifierParser.new.parse(
              value, runtime: @runtime, row_modifier:, cell_modifier:
            )

            ::CSVPlusPlus::Cell.new(@runtime.row_index, cell_index, parsed_value, cell_modifier)
          end

        ::CSVPlusPlus::Row.new(@runtime.row_index, cells, row_modifier)
      end

      # workflow when resolving static variable definitions
      def resolve_static_variables!(code_section)
        @scope.resolve_static_variables(code_section.variables, @runtime)
      end

      # workflow when resolving the values of all cells
      def resolve_all_cells!(template)
        workflow(log_subject: 'resolving all cell value variable references') do
          @runtime.map_rows(template.rows, cells_too: true) do |cell|
            cell.ast = @scope.resolve_cell_value if cell.ast
          end
        end
      end

      # workflow when resolving functions
      def apply_all_functions!(_template)
        workflow(log_subject: 'applying functions') do
          # XXX
        end
      end

      # workflow when writing results
      def outputting!(&block)
        workflow(log_subject: 'writing the spreadsheet') do
          block.call
        end
      end

      # to_s
      def to_s
        "Compiler(options: #{@options}, runtime: #{@runtime}, scope: #{@scope})"
      end

      # Log a message when in verbose mode
      def log(message)
        return unless @options.verbose

        # TODO: include line_number and other info if we have it
        warn("csv++: #{message}")
      end

      private

      # workflow when parsing the code section
      def parsing_code_section(&block)
        workflow(
          log_subject: 'parsing code section',
          processing_code_section: true
        ) do
          csv_section = block.call(@runtime.input)
          @runtime.rewrite_input!(csv_section)
        end
      end

      # workflow when expanding rows
      def expanding(&block)
        workflow(log_subject: 'expanding rows') do
          block.call
        end
      end

      def before_workflow!(log_subject, processing_code_section)
        log("Started #{log_subject}")
        @runtime.init!(processing_code_section ? 1 : (@runtime.length_of_code_section || 1))
      end

      def after_workflow!(log_subject)
        @runtime.unset!
        log("Finished #{log_subject}")
      end

      # TODO: we could add a progress loader here... but hopefully it never gets so slow
      # to warrant that
      def workflow(log_subject:, processing_code_section: false)
        before_workflow!(log_subject, processing_code_section)
        yield.tap { after_workflow!(log_subject) }
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end

# frozen_string_literal: true

require 'csv'
# TODO: move some of these out to csv_plus_plus.rb
require_relative '../cell'
require_relative '../modifier'
require_relative '../modifier.tab'
require_relative '../row'
require_relative '../template'
require_relative 'benchmarked_compiler'
require_relative 'code_section.tab'
require_relative 'entities'
require_relative 'runtime'
require_relative 'scope'

module CSVPlusPlus
  module Language
    # Encapsulates the parsing and building of objects (+Template+ -> +Row+ -> +Cell+). Variable resolution is delegated
    # to the +Scope+
    #
    # @attr_reader options [Options] The +Options+ to compile with
    # @attr_reader runtime [Runtime] The runtime execution
    # @attr_reader scope [Scope] +Scope+ for variable resolution
    class Compiler
      attr_reader :timings, :benchmark, :options, :runtime, :scope

      # Create a compiler and make sure it gets cleaned up
      #
      # @param input [String]
      # @param filename [String]
      # @param options [Options]
      # rubocop:disable Metrics/MethodLength
      def self.with_compiler(input:, filename:, options:, &block)
        runtime = ::CSVPlusPlus::Language::Runtime.new(filename:, input:)

        compiler = new(options:, runtime:)
        if options.verbose
          ::CSVPlusPlus::Language::BenchmarkedCompiler.with_benchmarks(compiler) do |c|
            block.call(c)
          end
        else
          yield(compiler)
        end
      ensure
        runtime.cleanup!
      end
      # rubocop:enable Metrics/MethodLength

      # @param runtime [Runtime]
      # @param options [Options]
      # @param scope [Scope, nil]
      def initialize(runtime:, options:, scope: nil)
        @options = options
        @runtime = runtime
        @scope = scope || ::CSVPlusPlus::Language::Scope.new(runtime:)
      end

      # Write the compiled results
      def outputting!
        @runtime.start_at_csv!
        yield
      end

      # Compile a template and return a +::CSVPlusPlus::Template+ instance ready to be written with a +Writer+
      #
      # @return [Template]
      def compile_template
        parse_code_section!
        rows = parse_csv_section!

        ::CSVPlusPlus::Template.new(rows:).tap do |t|
          t.validate_infinite_expands(@runtime)
          expanding { t.expand_rows! }
          resolve_all_cells!(t)
        end
      end

      # @return [String]
      def to_s
        "Compiler(options: #{@options}, runtime: #{@runtime}, scope: #{@scope})"
      end

      protected

      # Parses the input file and returns a +CodeSection+
      #
      # @return [CodeSection]
      def parse_code_section!
        @runtime.start!
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

      # Parse the CSV section and return an array of +Row+s
      #
      # @return [Array<Row>]
      def parse_csv_section!
        @runtime.start_at_csv!
        @runtime.map_rows(::CSV.new(runtime.input)) do |csv_row|
          parse_row(csv_row)
        end
      ensure
        # we're done with the file and everything is in memory
        @runtime.cleanup!
      end

      # Iterates through each cell of each row and resolves it's variable and function references.
      #
      # @param template [Template]
      # @return [Array<Entity>]
      def resolve_all_cells!(template)
        @runtime.start_at_csv!
        @runtime.map_rows(template.rows, cells_too: true) do |cell|
          cell.ast = @scope.resolve_cell_value if cell.ast
        end
      end

      # Expanding rows
      def expanding
        @runtime.start_at_csv!
        yield
      end

      private

      def parsing_code_section
        csv_section = yield(@runtime.input.read)
        @runtime.rewrite_input!(csv_section)
      end

      # Using the current +@runtime+ and the given +csv_row+ parse it into a +Row+ of +Cell+s
      # +csv_row+ should have already been run through a CSV parser and is an array of strings
      #
      # @param csv_row [Array<Array<String>>]
      # @return [Row]
      def parse_row(csv_row)
        row_modifier = ::CSVPlusPlus::Modifier.new(row_level: true)

        cells =
          @runtime.map_row(csv_row) do |value, _cell_index|
            cell_modifier = ::CSVPlusPlus::Modifier.new
            parsed_value = ::CSVPlusPlus::ModifierParser.new(row_modifier:, cell_modifier:).parse(value, @runtime)

            ::CSVPlusPlus::Cell.parse(parsed_value, runtime:, modifier: cell_modifier)
          end

        ::CSVPlusPlus::Row.new(@runtime.row_index, cells, row_modifier)
      end
    end
  end
end

# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Encapsulates the parsing and building of objects (+Template+ -> +Row+ -> +Cell+). Variable resolution is delegated
  # to the +Scope+
  #
  # @attr_reader options [Options] The +Options+ to compile with
  # @attr_reader runtime [Runtime] The runtime execution
  # rubocop:disable Metrics/ClassLength
  class Compiler
    extend ::T::Sig

    sig { returns(::CSVPlusPlus::Options) }
    attr_reader :options

    sig { returns(::CSVPlusPlus::Runtime::Runtime) }
    attr_reader :runtime

    sig do
      params(
        options: ::CSVPlusPlus::Options,
        runtime: ::CSVPlusPlus::Runtime::Runtime,
        block: ::T.proc.params(arg0: ::CSVPlusPlus::Compiler).void
      ).void
    end
    # Create a compiler and make sure it gets cleaned up
    #
    # @param options [Options]
    # @param runtime [Runtime] The initial +Runtime+ for the compiler
    def self.with_compiler(options:, runtime:, &block)
      if options.verbose
        ::CSVPlusPlus::BenchmarkedCompiler.with_benchmarks(options:, runtime:) do |c|
          block.call(c)
        end
      else
        block.call(new(options:, runtime:))
      end
    ensure
      runtime.cleanup!
    end

    sig { params(options: ::CSVPlusPlus::Options, runtime: ::CSVPlusPlus::Runtime::Runtime).void }
    # @param options [Options]
    # @param runtime [Runtime]
    def initialize(options:, runtime:)
      @options = options
      @runtime = runtime

      # TODO: infer a type
      # allow user-supplied key/values to override anything global or from the code section
      @runtime.def_variables(
        options.key_values.transform_values { |v| ::CSVPlusPlus::Entities::String.new(v.to_s) }
      )
    end

    sig { params(benchmark: ::Benchmark::Report).void }
    # Attach a +Benchmark+ and a place to store timings to the compiler class.
    #
    # @param benchmark [Benchmark] A +Benchmark+ instance
    def benchmark=(benchmark)
      @benchmark = ::T.let(benchmark, ::T.nilable(::Benchmark::Report))
      @timings = ::T.let([], ::T.nilable(::T::Array[::Benchmark::Tms]))
    end

    sig { returns(::CSVPlusPlus::Template) }
    # Compile a template and return a +::CSVPlusPlus::Template+ instance ready to be written with a +Writer+
    #
    # @return [Template]
    def compile_template
      parse_code_section!
      rows = parse_csv_section!

      ::CSVPlusPlus::Template.new(rows:, runtime: @runtime).tap do |t|
        t.validate_infinite_expands(@runtime)
        expanding! { t.expand_rows! }
        bind_all_vars! { t.bind_all_vars!(@runtime) }
        resolve_all_cells!(t)
      end
    end

    sig { params(block: ::T.proc.params(runtime: ::CSVPlusPlus::Runtime::Runtime).void).void }
    # Write the compiled results
    def outputting!(&block)
      @runtime.start_at_csv! { block.call(@runtime) }
    end

    protected

    sig { void }
    # Parses the input file and sets variables on +@runtime+ as necessary
    def parse_code_section!
      @runtime.start! do
        # TODO: this flow can probably be refactored, it used to have more needs back when we had to
        # parse and save the code_section
        parsing_code_section do |input|
          csv_section = ::CSVPlusPlus::Parser::CodeSection.new.parse(input, @runtime)

          # return the csv_section to the caller because they're gonna re-write input with it
          next csv_section
        end
      end
    end

    sig { returns(::T::Array[::CSVPlusPlus::Row]) }
    # Parse the CSV section and return an array of +Row+s
    #
    # @return [Array<Row>]
    def parse_csv_section!
      @runtime.start_at_csv! do
        @runtime.map_lines(::CSV.new(::T.unsafe(@runtime.input))) do |csv_row|
          parse_row(::T.cast(csv_row, ::T::Array[::String]))
        end
      end
    ensure
      # we're done with the file and everything is in memory
      @runtime.cleanup!
    end

    sig { params(template: ::CSVPlusPlus::Template).returns(::T::Array[::T::Array[::CSVPlusPlus::Entities::Entity]]) }
    # Iterates through each cell of each row and resolves it's variable and function references.
    #
    # @param template [Template]
    #
    # @return [Array<Entity>]
    def resolve_all_cells!(template)
      @runtime.start_at_csv! do
        @runtime.map_all_cells(template.rows) do |cell|
          cell.ast = @runtime.resolve_cell_value if cell.ast
        end
      end
    end

    sig { params(block: ::T.proc.void).void }
    # Expanding rows
    def expanding!(&block)
      @runtime.start_at_csv! { block.call }
    end

    sig { params(block: ::T.proc.void).void }
    # Binding all [[var=]] directives
    def bind_all_vars!(&block)
      @runtime.start_at_csv! { block.call }
    end

    private

    sig { params(block: ::T.proc.params(arg0: ::String).returns(::String)).void }
    def parsing_code_section(&block)
      csv_section = block.call(::T.must(::T.must(@runtime.input).read))
      @runtime.rewrite_input!(csv_section)
    end

    sig { params(csv_row: ::T::Array[::String]).returns(::CSVPlusPlus::Row) }
    # Using the current +@runtime+ and the given +csv_row+ parse it into a +Row+ of +Cell+s
    # +csv_row+ should have already been run through a CSV parser and is an array of strings
    #
    # @param csv_row [Array<Array<String>>]
    #
    # @return [Row]
    def parse_row(csv_row)
      row_modifier = ::CSVPlusPlus::Modifier.new(@options, row_level: true)

      cells = @runtime.map_row(csv_row) { |value, _cell_index| parse_cell(value, row_modifier) }

      ::CSVPlusPlus::Row.new(cells:, index: @runtime.row_index, modifier: row_modifier)
    end

    sig { params(value: ::String, row_modifier: ::CSVPlusPlus::Modifier::Modifier).returns(::CSVPlusPlus::Cell) }
    def parse_cell(value, row_modifier)
      cell_modifier = ::CSVPlusPlus::Modifier.new(@options)
      parsed_value = ::CSVPlusPlus::Parser::Modifier.new(cell_modifier:, row_modifier:).parse(value, @runtime)

      ::CSVPlusPlus::Cell.parse(parsed_value, runtime:, modifier: cell_modifier)
    end
  end
  # rubocop:enable Metrics/ClassLength
end

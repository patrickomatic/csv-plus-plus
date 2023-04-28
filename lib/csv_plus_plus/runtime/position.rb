# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # Keeps track of the position in a file where the parser is.  The parser makes various passes over the input but
    # it always needs to track the same things (line number, cell/row index, current cell)
    #
    # @attr cell [Cell] The current cell being processed
    # @attr cell_index [Integer] The index of the current cell being processed (starts at 0)
    # @attr row_index [Integer] The index of the current row being processed (starts at 0)
    # @attr line_number [Integer] The line number of the original csvpp template (starts at 1)
    # rubocop:disable Metrics/ClassLength
    class Position
      extend ::T::Sig

      sig { params(cell: ::T.nilable(::CSVPlusPlus::Cell)).returns(::T.nilable(::CSVPlusPlus::Cell)) }
      attr_writer :cell

      sig { params(cell_index: ::T.nilable(::Integer)).returns(::T.nilable(::Integer)) }
      attr_writer :cell_index

      sig { params(line_number: ::T.nilable(::Integer)).returns(::T.nilable(::Integer)) }
      attr_writer :line_number

      sig { params(row_index: ::T.nilable(::Integer)).returns(::T.nilable(::Integer)) }
      attr_writer :row_index

      sig { params(input: ::String).void }
      # @param input [String]
      def initialize(input)
        rewrite_input!(input)
      end

      sig { returns(::CSVPlusPlus::Cell) }
      # The current cell index.  This will only be set when processing the CSV section
      #
      # @return [Cell]
      def cell
        @cell ||= ::T.let(nil, ::T.nilable(::CSVPlusPlus::Cell))
        assert_initted!(@cell)
      end

      sig { returns(::Integer) }
      # The current CSV cell index.
      #
      # This will only be set when processing the CSV section and will throw an exception otherwise.  It is up to the
      # caller (the compiler) to make sure it's called in the context of a compilation stage and/or a
      # +#map_row+/+#map_rows+/+#map_lines+
      #
      # @return [Integer]
      def cell_index
        @cell_index ||= ::T.let(nil, ::T.nilable(::Integer))
        assert_initted!(@cell_index)
      end

      sig { returns(::Integer) }
      # The current CSV row index.  This will only be set when processing the CSV section
      #
      # This will only be set when processing the CSV section and will throw an exception otherwise.  It is up to the
      # caller (the compiler) to make sure it's called in the context of a compilation stage and/or a
      # +#map_row+/+#map_rows+/+#map_lines+
      #
      # @return [Integer]
      def row_index
        @row_index ||= ::T.let(nil, ::T.nilable(::Integer))
        assert_initted!(@row_index)
      end

      sig { returns(::Integer) }
      # The current line number being processed.  The line number is based on the entire file, irregardless of if it's
      # parsing the code section or the CSV section
      #
      # This will only be set when processing the csvpp file and will throw an exception otherwise.  It is up to the
      # caller (the compiler) to make sure it's called in the context of a compilation stage and/or a
      # +#map_row+/+#map_rows+/+#map_lines+
      #
      # @return [Integer]
      def line_number
        @line_number ||= ::T.let(nil, ::T.nilable(::Integer))
        assert_initted!(@line_number)
      end

      sig { void }
      # Clean up the Tempfile we're using for parsing
      def cleanup!
        input&.close
        input&.unlink
      end

      sig { returns(::T.nilable(::Tempfile)) }
      # The currently available input for parsing.  The tmp state will be re-written
      # between parsing the code section and the CSV section
      #
      # @return [::Tempfile]
      def input
        @input ||= ::T.let(::Tempfile.new, ::T.nilable(::Tempfile))
      end

      sig do
        type_parameters(:I, :O).params(
          lines: ::T::Enumerable[::T.type_parameter(:I)],
          block: ::T.proc.params(args0: ::T.type_parameter(:I)).returns(::T.type_parameter(:O))
        ).returns(::T::Array[::T.type_parameter(:O)])
      end
      # Map over a csvpp file and keep track of line_number and row_index
      #
      # @param lines [Array]
      #
      # @return [Array]
      def map_lines(lines, &block)
        self.line_number = 1
        lines.map do |line|
          ret = block.call(line)
          next_line!
          ret
        end
      end

      sig do
        type_parameters(:I, :O)
          .params(
            row: ::T::Enumerable[::T.all(::T.type_parameter(:I), ::Object)],
            block: ::T.proc.params(
              cell: ::T.all(::T.type_parameter(:I), ::Object),
              index: ::Integer
            ).returns(::T.type_parameter(:O))
          )
          .returns(::T::Array[::T.type_parameter(:O)])
      end
      # Map over a single row and keep track of the cell and it's index
      #
      # @param row [Array<Cell>] The row to map each cell over
      #
      # @return [Array]
      def map_row(row, &block)
        row.map.with_index do |cell, index|
          self.cell_index = index
          self.cell = cell if cell.is_a?(::CSVPlusPlus::Cell)
          block.call(cell, index)
        end
      end

      sig do
        type_parameters(:O).params(
          rows: ::T::Enumerable[::CSVPlusPlus::Row],
          block: ::T.proc.params(row: ::CSVPlusPlus::Row).returns(::T.type_parameter(:O))
        ).returns(::T::Array[::T.type_parameter(:O)])
      end
      # Map over all rows and keep track of row and line numbers
      #
      # @param rows [Array<Row>] The rows to map over (and keep track of indexes)
      #
      # @return [Array]
      def map_rows(rows, &block)
        self.row_index = 0
        map_lines(rows) do |row|
          block.call(row)
        end
      end

      sig do
        type_parameters(:R)
          .params(rows: ::T::Enumerable[::CSVPlusPlus::Row],
                  block: ::T.proc.params(cell: ::CSVPlusPlus::Cell, index: ::Integer).returns(::T.type_parameter(:R)))
          .returns(::T::Array[::T::Array[::T.type_parameter(:R)]])
      end
      # Map over all +rows+ and over all of their +cells+, calling the +&block+ with each +Cell+
      #
      # @param rows [Array<Row>]
      #
      # @return [Array<Array>]
      # rubocop:disable Naming/BlockForwarding
      def map_all_cells(rows, &block)
        self.row_index = 0
        map_lines(rows) { |row| map_row(row.cells, &block) }
      end
      # rubocop:enable Naming/BlockForwarding

      sig { returns(::Integer) }
      # Return the current spreadsheet row number.  It parallels +@row_index+ but starts at 1.
      #
      # @return [Integer, nil]
      def rownum
        row_index + 1
      end

      sig do
        type_parameters(:R).params(block: ::T.proc.returns(::T.type_parameter(:R))).returns(::T.type_parameter(:R))
      end
      # Each time we run a parse on the input, reset the runtime state starting at the beginning of the file
      def start!(&block)
        @row_index = @cell_index = 0

        ret = block.call
        finish!
        ret
      end

      sig { params(data: ::String).void }
      # We mutate the input over and over. It's ok because it's just a Tempfile
      #
      # @param data [::String] The data to rewrite our input file to
      def rewrite_input!(data)
        input&.truncate(0)
        input&.write(data)
        input&.rewind
      end

      private

      sig do
        type_parameters(:R).params(runtime_value: ::T.nilable(::T.type_parameter(:R))).returns(::T.type_parameter(:R))
      end
      def assert_initted!(runtime_value)
        ::T.must_because(runtime_value) do
          'Runtime value accessed without an initialized runtime.  Make sure you call Runtime#start! or ' \
            'Runtime#start_at_csv! first.'
        end
      end

      sig { void }
      # Called to mark the trackers dirty.  It should be an error to use them outside of an initialized context.
      def finish!
        @line_number = nil
        @row_index = nil
        @cell_index = nil
        @cell = nil
      end

      sig { returns(::Integer) }
      # Increment state to the next line
      #
      # @return [Integer]
      def next_line!
        self.row_index += 1
        self.line_number += 1
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end

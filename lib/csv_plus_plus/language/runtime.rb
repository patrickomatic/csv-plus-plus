# frozen_string_literal: true

require_relative 'entities'
require_relative 'syntax_error'
require 'tempfile'

RUNTIME_VARIABLES = {
  rownum: ::CSVPlusPlus::Language::Entities::RuntimeValue.new(
    lambda { |r|
      ::CSVPlusPlus::Language::Entities::Number.new(r.row_index + 1)
    }
  ),
  cellnum: ::CSVPlusPlus::Language::Entities::RuntimeValue.new(
    lambda { |r|
      ::CSVPlusPlus::Language::Entities::Number.new(r.cell_index + 1)
    }
  )
}.freeze

module CSVPlusPlus
  module Language
    # The runtime state of the compiler (the current +line_number+/+row_index+, +cell+, etc)
    #
    # @attr_reader filename [String, nil] The filename that the input came from (mostly used for debugging since
    #   +filename+ can be +nil+ if it's read from stdin.
    # @attr_reader length_of_code_section [Integer] The length (count of lines) of the code section part of the original
    #   input.
    # @attr_reader length_of_csv_section [Integer] The length (count of lines) of the CSV part of the original csvpp
    #   input.
    # @attr_reader length_of_original_file [Integer] The length (count of lines) of the original csvpp input.
    #
    # @attr cell [Cell] The current cell being processed
    # @attr cell_index [Integer] The index of the current cell being processed (starts at 0)
    # @attr row_index [Integer] The index of the current row being processed (starts at 0)
    # @attr line_number [Integer] The line number of the original csvpp template (starts at 1)
    class Runtime
      attr_reader :filename, :length_of_code_section, :length_of_csv_section, :length_of_original_file

      attr_accessor :cell, :cell_index, :row_index, :line_number

      # @param input [String] The input to be parsed
      # @param filename [String, nil] The filename that the input came from (mostly used for debugging since +filename+
      #   can be +nil+ if it's read from stdin
      def initialize(input:, filename:)
        @filename = filename || 'stdin'

        init_input!(input)
        start!
      end

      # Map over an a csvpp file and keep track of line_number and row_index
      #
      # @param lines [Array]
      # @return [Array]
      def map_lines(lines, &block)
        @line_number = 1
        lines.map do |line|
          block.call(line).tap { next_line! }
        end
      end

      # Map over a single row and keep track of the cell and it's index
      #
      # @param row [Array<Cell>] The row to map each cell over
      # @return [Array]
      def map_row(row, &block)
        @cell_index = 0
        row.map.with_index do |cell, index|
          set_cell!(cell, index)
          block.call(cell, index)
        end
      end

      # Map over all rows and keep track of row and line numbers
      #
      # @param rows [Array<Row>] The rows to map over (and keep track of indexes)
      # @param cells_too [boolean] If the cells of each +row+ should be iterated over also.
      # @return [Array]
      def map_rows(rows, cells_too: false, &block)
        @row_index = 0
        map_lines(rows) do |row|
          if cells_too
            # it's either CSV or a Row object
            map_row((row.is_a?(::CSVPlusPlus::Row) ? row.cells : row), &block)
          else
            block.call(row)
          end
        end
      end

      # Increment state to the next line
      #
      # @return [Integer]
      def next_line!
        @row_index += 1 unless @row_index.nil?
        @line_number += 1
      end

      # Set the current cell and index
      #
      # @param cell [Cell] The current cell
      # @param cell_index [Integer] The index of the cell
      def set_cell!(cell, cell_index)
        @cell = cell
        @cell_index = cell_index
      end

      # Each time we run a parse on the input, reset the runtime state starting at the beginning of the file
      def start!
        @row_index = @cell_index = nil
        @line_number = 1
      end

      # Reset the runtime state starting at the CSV section
      def start_at_csv!
        # TODO: isn't the input re-written anyway without the code section? why do we need this?
        start!
        @line_number = @length_of_code_section || 1
      end

      # @return [String]
      def to_s
        "Runtime(cell: #{@cell}, row_index: #{@row_index}, cell_index: #{@cell_index})"
      end

      # get the current (entity) value of a runtime value
      #
      # @param var_id [String, Symbol] The Variable#id  of the variable being resolved.
      # @return [Entity]
      def runtime_value(var_id)
        if runtime_variable?(var_id)
          ::RUNTIME_VARIABLES[var_id.to_sym].resolve_fn.call(self)
        else
          raise_syntax_error('Undefined variable', var_id)
        end
      end

      # Is +var_id+ a runtime variable?  (it's a static variable otherwise)
      #
      # @param var_id [String, Symbol] The Variable#id to check if it's a runtime variable
      # @return [boolean]
      def runtime_variable?(var_id)
        ::RUNTIME_VARIABLES.key?(var_id.to_sym)
      end

      # Called when an error is encoutered during parsing.  It will construct a useful
      # error with the current +@row/@cell_index+, +@line_number+ and +@filename+
      #
      # @param message [String] A message relevant to why this error is being raised.
      # @param bad_input [String] The offending input that caused this error to be thrown.
      # @param wrapped_error [StandardError, nil] The underlying error that was raised (if it's not from our own logic)
      def raise_syntax_error(message, bad_input, wrapped_error: nil)
        raise(::CSVPlusPlus::Language::SyntaxError.new(message, bad_input, self, wrapped_error:))
      end

      # The currently available input for parsing.  The tmp state will be re-written
      # between parsing the code section and the CSV section
      #
      # @return [String]
      def input
        @tmp
      end

      # We mutate the input over and over. It's ok because it's just a Tempfile
      #
      # @param data [String] The data to rewrite our input file to
      def rewrite_input!(data)
        @tmp.truncate(0)
        @tmp.write(data)
        @tmp.rewind
      end

      # Clean up the Tempfile we're using for parsing
      def cleanup!
        return unless @tmp

        @tmp.close
        @tmp.unlink
        @tmp = nil
      end

      private

      def count_code_section_lines(lines)
        eoc = ::CSVPlusPlus::Lexer::END_OF_CODE_SECTION
        lines.include?(eoc) ? (lines.take_while { |l| l != eoc }).length + 1 : 0
      end

      def init_input!(input)
        lines = (input || '').split(/\s*\n\s*/)
        @length_of_original_file = lines.length
        @length_of_code_section = count_code_section_lines(lines)
        @length_of_csv_section = @length_of_original_file - @length_of_code_section

        # we're gonna take our input file, write it to a tmp file then each
        # step is gonna mutate that tmp file
        @tmp = ::Tempfile.new
        rewrite_input!(input)
      end
    end
  end
end

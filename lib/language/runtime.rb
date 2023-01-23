# frozen_string_literal: true

require_relative 'entities'
require_relative 'syntax_error'
require 'tempfile'

ENTITIES = ::CSVPlusPlus::Language::Entities

RUNTIME_VARIABLES = {
  rownum: ::ENTITIES::RuntimeValue.new(->(r) { ::ENTITIES::Number.new(r.row_index + 1) }),
  cellnum: ::ENTITIES::RuntimeValue.new(->(r) { ::ENTITIES::Number.new(r.cell_index + 1) })
}.freeze

module CSVPlusPlus
  module Language
    ##
    # The runtime state of the compiler (the current linenumber/row, cell, etc)
    class Runtime
      attr_reader :filename, :length_of_code_section, :length_of_csv_section, :length_of_original_file

      attr_accessor :cell, :cell_index, :row_index, :line_number

      # initialize
      def initialize(input:, filename:)
        @filename = filename || 'stdin'

        init_input!(input)
        init!(1)
      end

      # map over an unparsed file and keep track of line_number and row_index
      def map_lines(lines, &block)
        @line_number = 1
        lines.map do |line|
          block.call(line).tap { next_line! }
        end
      end

      # map over a single row and keep track of the cell and it's index
      def map_row(row, &block)
        @cell_index = 0
        row.map.with_index do |cell, index|
          set_cell!(cell, index)
          block.call(cell, index)
        end
      end

      # map over all rows and keep track of row and line numbers
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
      def next_line!
        @row_index += 1 unless @row_index.nil?
        @line_number += 1
      end

      # Set the current cell and index
      def set_cell!(cell, cell_index)
        @cell = cell
        @cell_index = cell_index
      end

      # Each time we run a parse on the input, call this so that the runtime state
      # is set to it's default values
      def init!(start_line_number_at)
        @row_index = @cell_index = nil
        @line_number = start_line_number_at
      end

      # to_s
      def to_s
        "Runtime(cell: #{@cell}, row_index: #{@row_index}, cell_index: #{@cell_index})"
      end

      # get the current (entity) value of a runtime value
      def runtime_value(var_id)
        if runtime_variable?(var_id)
          ::RUNTIME_VARIABLES[var_id.to_sym].resolve_fn.call(self)
        else
          raise_syntax_error('Undefined variable', var_id)
        end
      end

      # Is +var_id+ a runtime variable?  (it's a static variable otherwise)
      def runtime_variable?(var_id)
        ::RUNTIME_VARIABLES.key?(var_id.to_sym)
      end

      # Called when an error is encoutered during parsing.  It will construct a useful
      # error with the current +@row/@cell_index+, +@line_number+ and +@filename+
      def raise_syntax_error(message, bad_input, wrapped_error: nil)
        raise(::CSVPlusPlus::Language::SyntaxError.new(message, bad_input, self, wrapped_error:))
      end

      # The currently available input for parsing.  The tmp state will be re-written
      # between parsing the code section and the CSV section
      def input
        @tmp
      end

      # We mutate the input over and over. It's ok because it's just a Tempfile
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

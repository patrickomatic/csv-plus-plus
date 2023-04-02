# typed: true
# frozen_string_literal: true

module CSVPlusPlus
  module Runtime
    # The runtime state of the compiler (the current +line_number+/+row_index+, +cell+ being processed, etc) for parsing
    # a given file.  We take multiple runs through the input file for parsing so it's really convenient to have a
    # central place for these things to be managed.
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
    #
    # rubocop:disable Metrics/ClassLength
    class Runtime
      extend ::T::Sig
      include ::CSVPlusPlus::Runtime::CanDefineReferences
      include ::CSVPlusPlus::Runtime::CanResolveReferences

      sig { returns(::String) }
      attr_reader :filename

      sig { returns(::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Function]) }
      attr_reader :functions

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :length_of_code_section

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :length_of_csv_section

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :length_of_original_file

      sig { returns(::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity]) }
      attr_reader :variables

      sig { returns(::T.nilable(::CSVPlusPlus::Cell)) }
      attr_accessor :cell

      sig { returns(::Integer) }
      attr_accessor :cell_index

      sig { returns(::Integer) }
      attr_accessor :row_index

      sig { returns(::Integer) }
      attr_accessor :line_number

      sig do
        params(
          input: ::String,
          filename: ::T.nilable(::String),
          functions: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Function],
          variables: ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity]
        ).void
      end
      # @param input [String] The input to be parsed
      # @param filename [String, nil] The filename that the input came from (mostly used for debugging since +filename+
      #   can be +nil+ if it's read from stdin
      # @param functions [Hash<Symbol, Function>] Pre-defined functions
      # @param variables [Hash<Symbol, Entity>] Pre-defined variables
      def initialize(input:, filename: nil, functions: {}, variables: {})
        @filename = filename || 'stdin'
        @functions = functions
        @variables = variables

        init_input!(input)
        start!
      end

      sig { void }
      # Clean up the Tempfile we're using for parsing
      def cleanup!
        return unless @tmp

        @tmp.close
        @tmp.unlink
        @tmp = nil
      end

      sig { returns(::File) }
      # The currently available input for parsing.  The tmp state will be re-written
      # between parsing the code section and the CSV section
      #
      # @return [::String]
      def input
        @tmp
      end

      sig do
        type_parameters(:I, :O).params(
          lines: ::T::Array[::T.type_parameter(:I)],
          block: ::T.proc.params(args0: ::T.type_parameter(:I)).returns(::T.all(::T.type_parameter(:O), ::Object))
        ).returns(::T::Array[::T.type_parameter(:O)])
      end
      # Map over a csvpp file and keep track of line_number and row_index
      #
      # @param lines [Array]
      #
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
      #
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
      #
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

      # Return the current spreadsheet row number.  It parallels +@row_index+ but starts at 1.
      #
      # @return [Integer, nil]
      def rownum
        return if @row_index.nil?

        @row_index + 1
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
        @row_index = @cell_index = 0
        @line_number = 1
      end

      # Reset the runtime state starting at the CSV section
      def start_at_csv!
        # TODO: isn't the input re-written anyway without the code section? why do we need this?
        start!
        @line_number = @length_of_code_section || 1
      end

      # Is +fn_id+ a builtin function?
      #
      # @param var_id [::String, Symbol] The Function#id to check if it's a runtime variable
      #
      # @return [boolean]
      def builtin_function?(fn_id)
        ::CSVPlusPlus::Entities::Builtins::FUNCTIONS.key?(fn_id.to_sym)
      end

      # Is +var_id+ a builtin variable?
      #
      # @param var_id [::String, Symbol] The Variable#id to check if it's a runtime variable
      #
      # @return [boolean]
      def builtin_variable?(var_id)
        ::CSVPlusPlus::Entities::Builtins::VARIABLES.key?(var_id.to_sym)
      end

      # Called when an error is encoutered during parsing formulas (whether in the code section or a cell).  It will
      # construct a useful error with the current +@row/@cell_index+, +@line_number+ and +@filename+
      #
      # @param message [String] A message relevant to why this error is being raised.
      # @param bad_input [String] The offending input that caused this error to be thrown.
      # @param wrapped_error [StandardError, nil] The underlying error that was raised (if it's not from our own logic)
      def raise_formula_syntax_error(message, bad_input, wrapped_error: nil)
        raise(::CSVPlusPlus::Error::FormulaSyntaxError.new(message, bad_input, self, wrapped_error:))
      end

      # Called when an error is encountered while parsing a modifier.
      def raise_modifier_syntax_error(message, bad_input, wrapped_error: nil)
        raise(::CSVPlusPlus::Error::ModifierSyntaxError.new(self, bad_input:, message:, wrapped_error:))
      end

      # We mutate the input over and over. It's ok because it's just a Tempfile
      #
      # @param data [::String] The data to rewrite our input file to
      def rewrite_input!(data)
        @tmp.truncate(0)
        @tmp.write(data)
        @tmp.rewind
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
    # rubocop:enable Metrics/ClassLength
  end
end

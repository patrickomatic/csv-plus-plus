# frozen_string_literal: true

require_relative './ast_builder'
require_relative './entity'

module CSVPlusPlus
  module Entities
    # A reference to a cell. Internally it is represented by a simple +cell_index+ and +row_index+ but there are
    # functions for converting to and from A1-style formats.  Supported formats are:
    #
    # * `1` - A reference to the entire first row
    # * `A` - A reference to the entire first column
    # * `A1` - A reference to the first cell (top left)
    # * `A1:D10` - The range defined between A1 and D10
    # * `Sheet1!B2` - Cell B2 on the sheet "Sheet1"
    #
    # @attr sheet_name [String, nil] The name of the sheet reference
    # @attr_reader cell_index [Integer, nil] The cell index of the cell being referenced
    # @attr_reader row_index [Integer, nil] The row index of the cell being referenced
    # @attr_reader scoped_to_expand [Expand, nil] If set, the expand in which this variable is scoped to. It cannot be
    #   resolved outside of the given expand.
    # @attr_reader upper_cell_index [Integer, nil] If set, the cell reference is a range and this is the upper cell
    #   index of it
    # @attr_reader upper_row_index [Integer, nil] If set, the cell reference is a range and this is the upper row index
    #   of it
    class CellReference < Entity
      attr_accessor :sheet_name
      attr_reader :cell_index, :row_index, :scoped_to_expand, :upper_cell_index, :upper_row_index

      # TODO: this is getting gross, maybe define an actual parser
      A1_NOTATION_REGEXP = /
        ^
          (?:
            (?:
              (?:'([^'\\]|\\.)*') # allow for a single-quoted sheet name
              |
              (\w+)               # or if it's not quoted, just allow \w+
            )
            !                     # if a sheet name is specified, it's always followed by a !
          )?
          ([a-zA-Z0-9]+)          # the only part required - something alphanumeric
          (?: :([a-zA-Z0-9]+))?   # and they might make it a range
        $
      /x
      public_constant :A1_NOTATION_REGEXP

      ALPHA = ('A'..'Z').to_a.freeze
      private_constant :ALPHA

      # Does the given +cell_reference_string+ conform to a valid cell reference?
      #
      # {https://developers.google.com/sheets/api/guides/concepts}
      #
      # @param cell_reference_string [::String] The string to check if it is a valid cell reference (we assume it's in
      #   A1 notation but maybe can support R1C1)
      #
      # @return [boolean]
      def self.valid_cell_reference?(cell_reference_string)
        !(cell_reference_string =~ ::CSVPlusPlus::Entities::CellReference::A1_NOTATION_REGEXP).nil?
      end

      # Either +ref+, +cell_index+ or +row_index+ must be specified.
      #
      # @param cell_index [Integer, nil] The index of the cell being referenced.
      # @param ref [Integer, nil] An A1-style cell reference (that will be parsed into it's row/cell indexes).
      # @param row_index [Integer, nil] The index of the row being referenced.
      # @param scoped_to_expand [Expand] The [[expand]] that this cell reference will be scoped to. In other words, it
      #   will only be able to be resolved if the runtime is within the bounds of the expand (it can't be referenced
      #   outside of the expand.)
      def initialize(ref: nil, cell_index: nil, row_index: nil, scoped_to_expand: nil)
        raise(::ArgumentError, 'Must specify :ref, :cell_index or :row_index') unless ref || cell_index || row_index

        super(:cell_reference)

        if ref
          from_a1_ref!(ref)
        else
          @cell_index = cell_index
          @row_index = row_index
        end

        @scoped_to_expand = scoped_to_expand
      end

      # @param other [Entity]
      #
      # @return [boolean]
      def ==(other)
        super && @cell_index == other.cell_index && @row_index == other.row_index && @sheet_name == other.sheet_name \
          && @scoped_to_expand == other.scoped_to_expand && @upper_cell_index == other.upper_cell_index \
          && @upper_row_index == other.upper_row_index
      end

      # Get the A1-style cell reference
      #
      # @param runtime [Runtime] The current runtime
      #
      # @return [::String] An A1-style reference
      def evaluate(runtime)
        unless in_scope?(runtime)
          runtime.raise_modifier_syntax_error(message: 'Reference is out of scope', bad_input: runtime.cell.value)
        end

        to_a1_ref
      end

      # Is the cell_reference a range? - something like A1:D10
      #
      # @return [boolean]
      def range?
        !upper_row_index.nil? || !upper_cell_index.nil?
      end

      private

      # A +CellReference+ can be bound to an expand, and in that case it is only in scope within the rows of that
      # expand.
      #
      # @param runtime [Runtime] The current runtime.
      #
      # @return [boolean]
      def in_scope?(runtime)
        @scoped_to_expand.nil? || runtime.in_scope?(@scoped_to_expand)
      end

      # Turns index-based/X,Y coordinates into a A1 format
      #
      # @return [::String]
      def to_a1_ref
        return unless @row_index || @cell_index

        rowref = @row_index ? (@row_index + 1).to_s : ''
        cellref = @cell_index ? to_a1_cell_ref : ''
        [cellref, rowref].join
      end

      # Turns a cell index into an A1 reference (just the "A" part - for example 0 == 'A', 1 == 'B', 2 == 'C', etc.)
      #
      # @return [::String]
      def to_a1_cell_ref
        c = @cell_index.dup
        ref = ''

        while c >= 0
          # rubocop:disable Lint/ConstantResolution
          ref += ALPHA[c % 26]
          # rubocop:enable Lint/ConstantResolution
          c = (c / 26).floor - 1
        end

        ref.reverse
      end

      def from_a1_ref!(ref)
        quoted_sheet_name, unquoted_sheet_name, lower_range, upper_range = ref.strip.match(
          ::CSVPlusPlus::Entities::CellReference::A1_NOTATION_REGEXP
        ).captures

        @sheet_name = quoted_sheet_name || unquoted_sheet_name

        parse_lower_range!(lower_range)
        parse_upper_range!(upper_range) if upper_range
      end

      def parse_lower_range!(lower_range)
        cell_ref, row_ref = lower_range.match(/^([a-zA-Z]+)?(\d+)?$/).captures
        @cell_index = from_a1_cell_ref!(cell_ref) if cell_ref
        @row_index = Integer(row_ref, 10) - 1 if row_ref
      end

      # TODO: make this less redundent with the above function
      def parse_upper_range!(upper_range)
        cell_ref, row_ref = upper_range.match(/^([a-zA-Z]+)?(\d+)?$/).captures
        @upper_cell_index = from_a1_cell_ref!(cell_ref) if cell_ref
        @upper_row_index = Integer(row_ref, 10) - 1 if row_ref
      end

      def from_a1_cell_ref!(cell_ref)
        (cell_ref.upcase.chars.reduce(0) do |cell_index, letter|
          # rubocop:disable Lint/ConstantResolution
          (cell_index * 26) + ALPHA.find_index(letter) + 1
          # rubocop:enable Lint/ConstantResolution
        end) - 1
      end
    end
  end
end

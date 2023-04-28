# typed: strict
# frozen_string_literal: true

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
    # rubocop:disable Metrics/ClassLength
    class CellReference < ::CSVPlusPlus::Entities::Entity
      extend ::T::Sig

      sig { returns(::T.nilable(::String)) }
      attr_accessor :sheet_name

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :cell_index

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :row_index

      sig { returns(::T.nilable(::CSVPlusPlus::Modifier::Expand)) }
      attr_reader :scoped_to_expand

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :upper_cell_index

      sig { returns(::T.nilable(::Integer)) }
      attr_reader :upper_row_index

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

      ALPHA = ::T.let(('A'..'Z').to_a.freeze, ::T::Array[::String])
      private_constant :ALPHA

      sig { params(cell_reference_string: ::String).returns(::T::Boolean) }
      # Does the given +cell_reference_string+ conform to a valid cell reference?
      #
      # {https://developers.google.com/sheets/api/guides/concepts}
      #
      # @param cell_reference_string [::String] The string to check if it is a valid cell reference (we assume it's in
      #   A1 notation but maybe can support R1C1)
      #
      # @return [::T::Boolean]
      def self.valid_cell_reference?(cell_reference_string)
        !(cell_reference_string =~ ::CSVPlusPlus::Entities::CellReference::A1_NOTATION_REGEXP).nil?
      end

      sig do
        params(
          cell_index: ::T.nilable(::Integer),
          ref: ::T.nilable(::String),
          row_index: ::T.nilable(::Integer),
          scoped_to_expand: ::T.nilable(::CSVPlusPlus::Modifier::Expand)
        ).void
      end
      # Either +ref+, +cell_index+ or +row_index+ must be specified.
      #
      # @param cell_index [Integer, nil] The index of the cell being referenced.
      # @param ref [Integer, nil] An A1-style cell reference (that will be parsed into it's row/cell indexes).
      # @param row_index [Integer, nil] The index of the row being referenced.
      # @param scoped_to_expand [Expand] The [[expand]] that this cell reference will be scoped to. In other words, it
      #   will only be able to be resolved if the position is within the bounds of the expand (it can't be referenced
      #   outside of the expand.)
      # rubocop:disable Metrics/MethodLength
      def initialize(cell_index: nil, ref: nil, row_index: nil, scoped_to_expand: nil)
        super()

        raise(::ArgumentError, 'Must specify :ref, :cell_index or :row_index') unless ref || cell_index || row_index

        if ref
          from_a1_ref!(ref)
        else
          @cell_index = ::T.let(cell_index, ::T.nilable(::Integer))
          @row_index = ::T.let(row_index, ::T.nilable(::Integer))

          @upper_cell_index = ::T.let(nil, ::T.nilable(::Integer))
          @upper_row_index = ::T.let(nil, ::T.nilable(::Integer))
        end

        @scoped_to_expand = scoped_to_expand
      end
      # rubocop:enable Metrics/MethodLength

      sig { override.params(other: ::BasicObject).returns(::T::Boolean) }
      # @param other [BasicObject]
      #
      # @return [boolean]
      def ==(other)
        case other
        when self.class
          @cell_index == other.cell_index && @row_index == other.row_index && @sheet_name == other.sheet_name \
            && @scoped_to_expand == other.scoped_to_expand && @upper_cell_index == other.upper_cell_index \
            && @upper_row_index == other.upper_row_index
        else
          false
        end
      end
      sig { override.params(position: ::CSVPlusPlus::Runtime::Position).returns(::String) }
      # Get the A1-style cell reference
      #
      # @param position [Position] The current position
      #
      # @return [::String] An A1-style reference
      def evaluate(position)
        # unless in_scope?(position)
        #   raise(::CSVPlusPlus::Error::ModifierSyntaxError.new(
        #     'Reference is out of scope',
        #     bad_input: position.cell.value)
        # end

        to_a1_ref(position) || ''
      end

      private

      sig { params(position: ::CSVPlusPlus::Runtime::Position).returns(::T.nilable(::String)) }
      # Turns index-based/X,Y coordinates into a A1 format
      #
      # @param position [Position]
      #
      # @return [::String, nil]
      def to_a1_ref(position)
        row_index = position_row_index(position)
        return unless row_index || @cell_index

        rowref = row_index ? (row_index + 1).to_s : ''
        cellref = @cell_index ? to_a1_cell_ref : ''
        [cellref, rowref].join
      end

      sig { params(position: ::CSVPlusPlus::Runtime::Position).returns(::T.nilable(::Integer)) }
      def position_row_index(position)
        @scoped_to_expand ? position.row_index : @row_index
      end

      sig { returns(::String) }
      # Turns a cell index into an A1 reference (just the "A" part - for example 0 == 'A', 1 == 'B', 2 == 'C', etc.)
      #
      # @return [::String]
      def to_a1_cell_ref
        c = @cell_index.dup
        ref = ''

        while c >= 0
          # rubocop:disable Lint/ConstantResolution
          ref += ::T.must(ALPHA[c % 26])
          # rubocop:enable Lint/ConstantResolution
          c = (c / 26).floor - 1
        end

        ref.reverse
      end

      sig { params(ref: ::String).void }
      def from_a1_ref!(ref)
        quoted_sheet_name, unquoted_sheet_name, lower_range, upper_range = ::T.must(
          ref.strip.match(
            ::CSVPlusPlus::Entities::CellReference::A1_NOTATION_REGEXP
          )
        ).captures

        @sheet_name = quoted_sheet_name || unquoted_sheet_name

        parse_lower_range!(lower_range) if lower_range
        parse_upper_range!(upper_range) if upper_range
      end

      sig { params(lower_range: ::String).void }
      def parse_lower_range!(lower_range)
        cell_ref, row_ref = ::T.must(lower_range.match(/^([a-zA-Z]+)?(\d+)?$/)).captures
        @cell_index = from_a1_cell_ref!(cell_ref) if cell_ref
        @row_index = Integer(row_ref, 10) - 1 if row_ref
      end

      sig { params(upper_range: ::String).void }
      # TODO: make this less redundant with the above function
      def parse_upper_range!(upper_range)
        cell_ref, row_ref = ::T.must(upper_range.match(/^([a-zA-Z]+)?(\d+)?$/)).captures
        @upper_cell_index = from_a1_cell_ref!(cell_ref) if cell_ref
        @upper_row_index = Integer(row_ref, 10) - 1 if row_ref
      end

      sig { params(cell_ref: ::String).returns(::Integer) }
      def from_a1_cell_ref!(cell_ref)
        (cell_ref.upcase.chars.reduce(0) do |cell_index, letter|
          # rubocop:disable Lint/ConstantResolution
          (cell_index * 26) + ::T.must(ALPHA.find_index(letter)) + 1
          # rubocop:enable Lint/ConstantResolution
        end) - 1
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end

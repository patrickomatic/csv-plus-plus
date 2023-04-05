# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # A row of a template.  A row contains an +Array+ of +Cell+s and possibly a row-level +Modifier+.
  #
  # @attr_reader cells [Array<Cell>] The cells contained by this row.
  # @attr_reader index [Integer] The index of this row. Starts at 0.
  # @attr_reader modifier [Modifier] The modifier to apply to all cells in this row
  class Row
    extend ::T::Sig

    sig { returns(::T::Array[::CSVPlusPlus::Cell]) }
    attr_reader :cells

    sig { returns(::Integer) }
    attr_reader :index

    sig { returns(::CSVPlusPlus::Modifier::Modifier) }
    attr_reader :modifier

    sig do
      params(cells: ::T::Array[::CSVPlusPlus::Cell], index: ::Integer, modifier: ::CSVPlusPlus::Modifier::Modifier).void
    end
    # @param cells [Array<Cell>] The cells belonging to this row
    # @param index [Integer] The index of this row (starts at 0)
    # @param modifier [Modifier] The modifier to apply to all cells in this row
    def initialize(cells:, index:, modifier:)
      @cells = cells
      @modifier = modifier
      @index = index
    end

    sig { params(index: ::Integer).void }
    # Set the row's +index+ and update the +row_index+ of all affected cells
    #
    # @param index [Integer] The index of this row (starts at 0)
    def index=(index)
      @index = index
      @cells.each { |cell| cell.row_index = index }
    end

    sig { returns(::Integer) }
    # How much this row will expand itself, if at all (0)
    #
    # @return [Integer]
    def expand_amount
      return 0 if @modifier.expand&.repetitions.nil?

      ::T.must(@modifier.expand).repetitions || (1000 - @index)
    end

    sig { params(starts_at: ::Integer, into: ::T::Array[::CSVPlusPlus::Row]).returns(::T::Array[::CSVPlusPlus::Row]) }
    # Starting at +starts_at+, do a deep copy of this row into the +Array+ referenced by +into+.
    #
    # @param starts_at [Integer] The +row_index+ where this row was expanded.
    # @param into [Array<Row>] An array where the expanded rows will be accumulated.
    #
    # @return [Array<Row>] The rows expanded
    def expand_rows(starts_at:, into: [])
      return into unless @modifier.expand

      ::T.must(@modifier.expand).starts_at = starts_at

      starts_at.upto(expand_amount + starts_at - 1) do |row_index|
        into << deep_clone.tap { |c| c.index = row_index }
      end
      into
    end

    sig { returns(::T::Boolean) }
    # Does the row have an ![[expand]] modifier but is yet to be expanded?
    #
    # @return [boolean]
    def unexpanded?
      return true unless @modifier.expand

      !::T.must(@modifier.expand).expanded?
    end

    private

    sig { returns(::CSVPlusPlus::Row) }
    # Return a deep copy of this row
    #
    # @return [Row]
    def deep_clone
      ::T.cast(::Marshal.load(::Marshal.dump(self)), ::CSVPlusPlus::Row)
    end
  end
end

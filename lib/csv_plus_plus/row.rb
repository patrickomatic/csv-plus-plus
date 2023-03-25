# frozen_string_literal: true

module CSVPlusPlus
  # A row of a template.  A row contains an +Array+ of +Cell+s and possibly a row-level +Modifier+.
  #
  # @attr_reader cells [Array<Cell>] The cells contained by this row.
  # @attr_reader index [Integer] The index of this row. Starts at 0.
  # @attr_reader modifier [Modifier] The modifier to apply to all cells in this row
  class Row
    attr_reader :cells, :index, :modifier

    # @param index [Integer] The index of this row (starts at 0)
    # @param cells [Array<Cell>] The cells belonging to this row
    # @param modifier [Modifier] The modifier to apply to all cells in this row
    def initialize(index, cells, modifier)
      @cells = cells
      @modifier = modifier
      @index = index
    end

    # Set the row's +index+ and update the +row_index+ of all affected cells
    #
    # @param index [Integer] The index of this row (starts at 0)
    def index=(index)
      @index = index
      @cells.each { |cell| cell.row_index = index }
    end

    # How much this row will expand itself, if at all (0)
    #
    # @return [Integer]
    def expand_amount
      return 0 unless @modifier.expand

      @modifier.expand.repetitions || (1000 - @index)
    end

    # Starting at +starts_at+, do a deep copy of this row into the +Array+ referenced by +into+.
    #
    # @param into [Array<Row>] An array where the expanded rows will be accumulated.
    # @param start_at [Integer] The +row_index+ where this row was expanded.
    #
    # @return [Integer] The amount of rows expanded
    def expand_rows(starts_at:, into: [])
      return unless @modifier.expand

      @modifier.expand.starts_at = starts_at

      starts_at.upto(expand_amount + starts_at - 1) do |row_index|
        into << deep_clone.tap { |c| c.index = row_index }
      end
      into
    end

    # Does the row have an ![[expand]] modifier but is yet to be expanded?
    #
    # @return [boolean]
    def unexpanded?
      !@modifier.expand.nil? && !@modifier.expand.expanded?
    end

    private

    # Return a deep copy of this row
    #
    # @return [Row]
    def deep_clone
      ::Marshal.load(::Marshal.dump(self))
    end
  end
end

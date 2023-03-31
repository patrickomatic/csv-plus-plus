# typed: false
# frozen_string_literal: true

module CSVPlusPlus
  # A cell of a template
  #
  # @attr ast [Entity]
  # @attr row_index [Integer] The cell's row index (starts at 0)
  # @attr_reader index [Integer] The cell's index (starts at 0)
  # @attr_reader modifier [Modifier] The modifier for this cell
  class Cell
    attr_accessor :ast, :row_index
    attr_reader :index, :modifier

    # Parse a +value+ into a Cell object.
    #
    # @param value [String] A string value which should already have been processed through a CSV parser
    # @param runtime [Runtime]
    # @param modifier [Modifier]
    #
    # @return [Cell]
    def self.parse(value, runtime:, modifier:)
      new(value:, row_index: runtime.row_index, index: runtime.cell_index, modifier:).tap do |c|
        c.ast = ::CSVPlusPlus::Parser::CellValue.new.parse(value, runtime)
      end
    end

    # @param index [Integer] The cell's index (starts at 0)
    # @param modifier [Modifier] A modifier to apply to this cell
    # @param row_index [Integer] The cell's row index (starts at 0)
    # @param value [String] A string value which should already have been processed through a CSV parser
    def initialize(row_index:, index:, value:, modifier:)
      @value = value
      @modifier = modifier
      @index = index
      @row_index = row_index
    end

    # The +@value+ (cleaned up some)
    #
    # @return [String]
    # TODO: is this used?
    def value
      return if @value.nil? || @value.strip.empty?

      @value.strip
    end

    # A compiled final representation of the cell.  This can only happen after all cell have had variables and functions
    # resolved.
    #
    # @param runtime [Runtime]
    #
    # @return [String]
    def evaluate(runtime)
      return value unless @ast

      "=#{@ast.evaluate(runtime)}"
    end
  end
end

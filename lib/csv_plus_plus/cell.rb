# frozen_string_literal: true

require_relative 'modifier'
require_relative 'parser/cell_value.tab'

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

    # @param row_index [Integer] The cell's row index (starts at 0)
    # @param index [Integer] The cell's index (starts at 0)
    # @param value [String] A string value which should already have been processed through a CSV parser
    # @param modifier [Modifier] A modifier to apply to this cell
    def initialize(row_index:, index:, value:, modifier:)
      @value = value
      @modifier = modifier
      @index = index
      @row_index = row_index
    end

    # The +@value+ (cleaned up some)
    #
    # @return [String]
    def value
      return if @value.nil? || @value.strip.empty?

      @value.strip
    end

    # @return [String]
    def to_s
      "Cell(index: #{@index}, row_index: #{@row_index}, value: #{@value}, modifier: #{@modifier})"
    end

    # A compiled final representation of the cell.  This can only happen after all cell have had
    # variables and functions resolved.
    #
    # @return [String]
    def to_csv
      return value unless @ast

      # This looks really simple but we're relying on each node of the AST to define #to_s such that calling
      # this at the top will recursively print the tree (as a well-formatted spreadsheet formula)
      "=#{@ast}"
    end
  end
end

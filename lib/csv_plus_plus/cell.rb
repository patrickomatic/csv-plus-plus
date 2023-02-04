# frozen_string_literal: true

require_relative './language/cell_value.tab'
require_relative './modifier'

module CSVPlusPlus
  ##
  # A cell of a template
  class Cell
    attr_accessor :ast, :row_index
    attr_reader :index, :modifier

    # Parse a +value+ into a Cell object.  The +value+ should already have been through
    # a CSV parser
    def self.parse(value, runtime:, modifier:)
      new(value:, row_index: runtime.row_index, index: runtime.cell_index, modifier:).tap do |c|
        c.ast = ::CSVPlusPlus::Language::CellValueParser.new.parse(value, runtime)
      end
    end

    # initialize
    def initialize(row_index:, index:, value:, modifier:)
      @value = value
      @modifier = modifier
      @index = index
      @row_index = row_index
    end

    # The value (cleaned up some)
    def value
      return if @value.nil? || @value.strip.empty?

      @value.strip
    end

    # to_s
    def to_s
      "Cell(index: #{@index}, row_index: #{@row_index}, value: #{@value}, modifier: #{@modifier})"
    end

    # A compiled final representation of the cell.  This can only happen after all cell have had
    # variables and functions resolved.
    def to_csv
      return value unless @ast

      # This looks really simple but we're relying on each node of the AST to define #to_s and calling
      # this at the top will recursively print the tree
      "=#{@ast}"
    end
  end
end

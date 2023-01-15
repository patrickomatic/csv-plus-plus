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
    def self.parse(value, runtime:, index:, modifier:, row_index:)
      new(value, row_index, index, modifier).tap do |c|
        c.ast = ::CSVPlusPlus::CellValueParser.new.parse(value, runtime)
      end
    end

    # initialize
    def initialize(row_index, index, value, modifier)
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
      "Cell(index: #{index} row_index: #{row_index} value: #{value} modifier: #{modifier})"
    end

    # A compiled final representation of the cell.  This includes all variables and functions
    # resolved and ready to be output
    def to_csv
      return @value if @ast.nil?

      to_csv_dfs(@ast).join
    end

    private

    # rubocop:disable Metrics/MethodLength
    def to_csv_dfs(node, output: ['='], add_comma: false)
      output << node.to_s
      output << ', ' if add_comma

      if node.function_call?
        output << '('
        arg_length = node.arguments.length
        node.arguments.each_with_index do |n, i|
          to_csv_dfs(n, output:, add_comma: i < arg_length - 1)
        end
        output << ')'
      else
        output
      end
    end
    # rubocop:enable Metrics/MethodLength
  end
end

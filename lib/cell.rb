require_relative './modifier'
#require_relative './language/ast'
require_relative './language/cell_value.tab'

module CSVPlusPlus
  class Cell
    attr_accessor :ast, :row_index
    attr_reader :index, :modifier

    def initialize(row_index, index, value, modifier)
      @value = value
      @modifier = modifier
      @index = index
      @row_index = row_index
    end

    def self.parse(value, execution_context:, index:, modifier:, row_index:)
      Cell.new(value, row_index, index, modifier).tap do |c| 
        c.ast = Language::CellValueParser.new.parse(value, execution_context)
      end
    end

    def value
      return nil if @value.nil? || @value.strip.empty?
      @value.strip
    end

    def to_s
      "Cell(index: #{index} row_index: #{row_index} value: #{value} modifier: #{modifier})"
    end

    def to_csv
      return @value if @ast.nil?
      to_csv_dfs(@ast).join('')
    end

    private

    def to_csv_dfs node, output: ['='], add_comma: false
      output << node.to_s
      output << ', ' if add_comma

      if node.type == :function_call
        output << '('
        arg_length = node.arguments.length
        node.arguments.each_with_index do |n, i|
          to_csv_dfs(n,
                     output:,
                     add_comma: i < arg_length - 1)
        end
        output << ')'
      end
      output
    end
  end
end

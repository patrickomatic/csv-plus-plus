require_relative 'cell_value.tab'
require_relative 'modifier'
require_relative 'ast'

module CSVPlusPlus
  class Cell
    attr_accessor :row_index
    attr_reader :ast, :index, :modifier

    def initialize(row_index, index, value, modifier)
      @value = value
      @ast = CellValueParser.new.parse(value) unless value.nil?
      @modifier = modifier
      @index = index
      @row_index = row_index
    end

    def interpolate_variables!(variables)
      return nil if @value.nil? || @ast.nil?
      @ast = AST::interpolate_variables(@ast, variables)
    end

    def value
      return nil if @value.nil? || @value.strip.empty?
      @value.strip
    end

    def to_csv
      return value if @ast.nil?

      argument_index = 0

      "=" + (AST::depth_first_search @ast do |node|
        type, value = node
        case type
        when :fn
          argument_index = 0
          "#{value}("
        when :after_fn
          ")"
        else
          argument_index += 1
          argument_index == 1 ? value : ", #{value}"
        end
      end).join('')
    end
  end
end

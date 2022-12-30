require_relative 'cell_value.tab'
require_relative 'modifier'
require_relative 'ast'

module CSVPlusPlus
  class Cell
    attr_reader :ast, :modifier

    def initialize(value, modifier = nil)
      @value = value
      @ast = CellValueParser.new.parse(value) unless value.nil?
      @modifier = modifier
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
      argument_index = nil
      "=" + (AST::depth_first_search @ast do |node|
        type, value = node
        case type
        when :fn
          argument_index = 0
          "#{value}("
        when :literal
          argument_index += 1
          argument_index == 1 ? value : ", #{value}"
        when :after_fn
          ")"
        end
      end).join('')
    end
  end
end

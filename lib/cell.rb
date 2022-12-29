require_relative 'cell_value_parser.tab'
require_relative 'modifier'
require_relative 'ast'

module GSPush
  class Cell
    attr_reader :modifier

    def initialize(value, modifier = nil)
      @value = value
      @modifier = modifier
    end

    def interpolate_variables!(variables)
      return nil if @value.nil?
      ast = CellValueParser.new.parse(@value)
      @value = AST.interpolate_variables(ast, variables)
    end

    def value
      return nil if @value.nil? || @value.strip.empty?
      @value.strip
    end

    def to_s
      "#{@value} #{@modifier}"
    end
  end
end

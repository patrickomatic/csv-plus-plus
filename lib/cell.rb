require_relative 'cell_value_parser.tab'
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
      return nil if @value.nil?
      @ast = AST.interpolate_variables(ast, variables)
    end

    def value
      return nil if @value.nil? || @value.strip.empty?
      @value.strip
    end

    def to_csv
      argument_index = nil
      str = "="
      AST::dfs(@ast) do |node|
        type, value = node
        case type
        when :fn
          str << "#{value}("
          argument_index = 0
        when :literal
          if argument_index == 0
            str << value
          else 
            str << ", #{value}"
          end
          argument_index += 1
        when :after_fn
          str << ")"
        end
      end
      str
    end
  end
end

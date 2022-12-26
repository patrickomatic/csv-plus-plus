require_relative 'modifier'

module GSPush
  class Cell
    attr_reader :modifier

    def initialize(value, modifier = nil, key_values: {}, functions: {})
      @value = value
      @modifier = modifier
      @key_values = key_values
      @functions = functions
    end

    def value 
      @value.dup.tap do |value|
        @key_values.each {|k, v| value[k] &&= v}
        @functions.each {|k, v| value[k] &&= v}
      end
    end

    def to_s
      "#{@value} #{@modifier}"
    end
  end
end

require_relative 'modifier'

module GSPush
  class Cell
    attr_reader :modifier

    def initialize(value, modifier = nil)
      @value = value
      @modifier = modifier
    end

    def interpolate_variables!(variables)
      return nil if @value.nil?
      variables.each do |k, v|
        @value.gsub!("$$#{k}", v.to_s)
      end
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

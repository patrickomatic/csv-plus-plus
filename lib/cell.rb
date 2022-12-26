require_relative 'modifier'

module GSPush
  class Cell
    attr_reader :modifier, :value

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

    def interpolate_functions!(functions)
      return nil if @value.nil?

      functions.each do |k, v|
        # XXX this is more complicated I think
        #@value["$$#{k}"] &&= v
      end
    end

    def to_s
      "#{@value} #{@modifier}"
    end
  end
end

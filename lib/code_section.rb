module GSPush
  class CodeSection
    attr_reader :variables, :functions

    def initialize(variables, functions)
      @variables = variables
      @functions = functions
    end

    def self.parse!(file)
      # XXX parse it
      return CodeSection.new({}, {}), File.read(file)
    end
  end
end

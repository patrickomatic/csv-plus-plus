require 'csv'
require_relative 'modifier'
require_relative 'row'

module GSPush
  class Template
    attr_accessor :rows

    def initialize(input, key_values: {}, verbose: false)
      @input = input
      @key_values = key_values
      @verbose = verbose
      @rows = []
    end

    def process!
      @rows = CSV.new(@input).map do |row|
        Row.parse_row(row)
      end
    end

    def get_all_values
      @rows
    end
  end
end

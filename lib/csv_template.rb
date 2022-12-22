require 'csv'

module GSPush
  class CSVTemplate
    def initialize(input, key_values, headers)
      @csv = CSV.new(input, headers)
      @key_values = key_values
    end

    def get_header
    end

    def get_all_values
      @csv.readlines
    end
  end
end

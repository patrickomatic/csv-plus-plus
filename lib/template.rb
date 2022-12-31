require 'csv'
require 'tempfile'
require_relative 'row'
require_relative 'code_section'

module CSVPlusPlus
  class Template
    attr_reader :rows

    def initialize(key_values: {}, verbose: false)
      @key_values = key_values
      @verbose = verbose
      @rows = []
    end

    def self.process!(input, key_values: {}, verbose: false)
      tmp = Tempfile.new

      begin
        tmp.write input
        tmp.rewind

        Template.new(key_values: key_values, verbose: verbose).tap do |t|
          code_section = CodeSection.parse!(tmp)
          t.parse_rows!(tmp)
          t.expand_rows!
          t.interpolate_variables!(code_section.variables)
        end
      ensure
        tmp.close
        tmp.unlink
      end
    end

    # TODO move this into a CsvSection?
    def parse_rows!(file)
      @rows = CSV.new(file).map.with_index(1) do |row, row_number|
        Row.parse_row(row, row_number)
      end
    end

    def expand_rows!
      # TODO we should have a check that you can only have one infinite expand
      expanded_rows = []
      @rows.each do |row|
        if !row.modifier.nil? && !row.modifier.expand.nil?
          # TODO make the 1000 agreed upon somewhere
          (row.modifier.expand.repetitions || (1000 - @rows.length)).times do
            expanded_rows = expanded_rows << Marshal.load(Marshal.dump(row))
          end
        else
          expanded_rows = expanded_rows << Marshal.load(Marshal.dump(row))
        end
      end

      @rows = expanded_rows
    end

    def interpolate_variables!(variables)
      @rows.each.with_index(1) do |row, row_number|
        row.cells.each do |cell|
          cell.interpolate_variables!({
            "rownum" => [:number, row_number],
            # TODO infer a type from the key_values
            **Hash[@key_values.map {|k, v| [k, [:unknown, v]]}],
            **variables,
          })
        end
      end
    end
  end
end

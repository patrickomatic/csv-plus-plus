require 'csv'
require 'tempfile'
require_relative 'modifier'
require_relative 'row'
require_relative 'spreadsheet'
require_relative 'code_section'

module GSPush
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
          t.parse_code_section!(tmp)
          t.parse_rows!(tmp)
          t.expand_rows!
          t.interpolate_variables!
        end
      ensure
        tmp.close
        tmp.unlink
      end
    end

    def parse_code_section!(file)
      code_section, csv_section = CodeSection.parse!(file)
      @code_section = code_section

      file.write csv_section
      file.rewind
    end

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
          (row.modifier.expand.repetitions || (Spreadsheet::SPREADSHEET_INFINITY - @rows.length)).times do
            expanded_rows = expanded_rows << Marshal.load(Marshal.dump(row))
          end
        else
          expanded_rows = expanded_rows << Marshal.load(Marshal.dump(row))
        end
      end

      @rows = expanded_rows
    end

    def interpolate_variables!
      @rows.each.with_index(1) do |row, row_number|
        row.cells.each do |cell|
          cell.interpolate_variables!({ rownum: row_number, **(@key_values || {}) })
        end
      end
    end
  end
end

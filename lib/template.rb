require 'csv'
require 'tempfile'
require_relative 'row'
require_relative 'code_section'
require_relative 'syntax_error'

module CSVPlusPlus
  class Template
    attr_reader :rows

    def initialize(key_values: {}, verbose: false, rows: [])
      @key_values = key_values
      @verbose = verbose
      @rows = rows
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

    def parse_rows!(file)
      infinite_expands = 0
      @rows = CSV.new(file).map.with_index do |row, row_number|
        row = Row.parse_row(row, row_number)

        infinite_expands += 1 if row.modifier.expand&.infinite?
        if infinite_expands > 1
          raise SyntaxError.new('You can only have one infinite expand= (on all others you must specify an amount)',
                                row_number:)
        end

        row
      end
    end

    def expand_rows!
      expanded_rows = []
      @rows.each do |row|
        if row.modifier.expand
          # TODO ideally we want to merge the contents of the expanded row to the rows below it.
          # an example is you wanted a formula to apply for the rest of the sheet but you also
          # wanted to supply the first couple rows for it underneath it's expand definition
          row.expand_amount.times { expanded_rows << row.deep_clone }
        else
          expanded_rows << row
        end
      end

      # expanding screwed up row indexes, so recalculate them
      expanded_rows.each_with_index do |row, index|
        row.index = index
      end

      @rows = expanded_rows
    end

    def interpolate_variables!(variables)
      @rows.each.with_index(1) do |row, row_number|
        row.cells.each do |cell|
          cell.interpolate_variables!({
            "rownum" => [:number, row_number],
            # TODO infer a type from the key_values
            **variables,
            # user-supplied key/values come last so they can override everything if the user wants
            **Hash[@key_values.map {|k, v| [k, [:unknown, v]]}],
          })
        end
      end
    end
  end
end

# frozen_string_literal: true

require_relative './file_backer_upper'

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to CSV
    class CSV < ::CSVPlusPlus::Writer::BaseWriter
      include ::CSVPlusPlus::Writer::FileBackerUpper

      # Write a +template+ to CSV
      #
      # @param template [Template] The template to use as input to be written.  It should have been compiled by calling
      #   Compiler#compile_template
      def write(template)
        # TODO: also read it and merge the results
        ::CSV.open(@options.output_filename, 'wb') do |csv|
          @runtime.map_rows(template.rows) do |row|
            csv << build_row(row)
          end
        end
      end

      private

      def build_row(row)
        @runtime.map_row(row.cells) { |cell, _i| cell.evaluate(@runtime) }
      end
    end
  end
end

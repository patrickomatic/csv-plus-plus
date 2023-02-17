# frozen_string_literal: true

require_relative './file_backer_upper'

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to CSV
    class CSV < ::CSVPlusPlus::Writer::BaseWriter
      include ::CSVPlusPlus::Writer::FileBackerUpper

      # write a +template+ to CSV
      def write(template)
        # TODO: also read it and merge the results
        ::CSV.open(@options.output_filename, 'wb') do |csv|
          template.rows.each do |row|
            csv << build_row(row)
          end
        end
      end

      protected

      def load_requires
        require('csv')
      end

      private

      def build_row(row)
        row.cells.map(&:to_csv)
      end
    end
  end
end

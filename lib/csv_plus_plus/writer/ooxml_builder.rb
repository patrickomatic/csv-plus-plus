# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Write to Office Open XML format
    class OoxmlBuilder
      # initialize
      def initialize(rows:)
        @rows = rows
      end

      # write the +template+ to OOXML
      def write(sheet_name, output_filename)
        ::Axlsx::Package.new do |p|
          # TODO: how do I get the worksheet by name (I guess I need a separate lib for reading)
          p.workbook.add_worksheet(name: sheet_name) do |sheet|
            @rows.each do |row|
              sheet.add_row(axlsx_row(row))
            end
          end

          p.serialize(output_filename)
        end
      end

      private

      def axlsx_row(row)
        row.cells.map(&:to_csv)
      end
    end
  end
end

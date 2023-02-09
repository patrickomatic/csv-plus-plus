# frozen_string_literal: true

require_relative './ooxml_builder'

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to an Excel file
    class Excel < ::CSVPlusPlus::Writer::BaseWriter
      # write a +template+ to an Excel file
      def write(template)
        ::CSVPlusPlus::Writer::OoxmlBuilder.new(rows: template.rows).write(
          @options.sheet_name,
          @options.output_filename
        )
      end

      protected

      def load_requires
        require('caxlsx')
      end
    end
  end
end

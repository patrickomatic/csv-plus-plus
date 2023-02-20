# frozen_string_literal: true

require_relative './file_backer_upper'
require_relative './rubyxl_builder'

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to an Excel file
    class Excel < ::CSVPlusPlus::Writer::BaseWriter
      include ::CSVPlusPlus::Writer::FileBackerUpper

      # write the +template+ to an Excel file
      def write(template)
        ::CSVPlusPlus::Writer::RubyXLBuilder.new(
          output_filename: @options.output_filename,
          rows: template.rows,
          sheet_name: @options.sheet_name
        ).write
      end
    end
  end
end

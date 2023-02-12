# frozen_string_literal: true

require_relative './rubyxl_builder'

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to an Excel file
    class Excel < ::CSVPlusPlus::Writer::BaseWriter
      # write the +template+ to an Excel file
      def write(template)
        ::CSVPlusPlus::Writer::RubyXLBuilder.new(output_filename: @options.output_filename, rows: template.rows).write(
          @options.sheet_name
        )
      end

      protected

      def load_requires
        require('rubyXL')
        require('rubyXL/convenience_methods')
      end
    end
  end
end

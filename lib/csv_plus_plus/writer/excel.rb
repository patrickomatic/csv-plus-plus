# typed: strict
# frozen_string_literal: true

require_relative './file_backer_upper'
require_relative './rubyxl_builder'

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to an Excel file
    class Excel < ::CSVPlusPlus::Writer::BaseWriter
      extend ::T::Sig

      include ::CSVPlusPlus::Writer::FileBackerUpper

      sig { override.params(template: ::CSVPlusPlus::Template).void }
      # Write the +template+ to an Excel file
      #
      # @param template [Template] The template to write
      def write(template)
        ::CSVPlusPlus::Writer::RubyXLBuilder.new(
          input_filename: ::T.must(@options.output_filename),
          rows: template.rows,
          runtime: @runtime,
          sheet_name: @options.sheet_name
        ).build_workbook.write(@options.output_filename)
      end
    end
  end
end

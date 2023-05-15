# typed: strict
# frozen_string_literal: true

require_relative './file_backer_upper'
require_relative './rubyxl_builder'

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to an Excel file
    class Excel < ::CSVPlusPlus::Writer::Writer
      extend ::T::Sig
      include ::CSVPlusPlus::Writer::FileBackerUpper

      sig { params(options: ::CSVPlusPlus::Options::FileOptions, position: ::CSVPlusPlus::Runtime::Position).void }
      # @param options [Options::FileOptions]
      # @param position [Runtime::Position]
      def initialize(options, position)
        super(position)

        @options = options
      end

      sig { override.params(template: ::CSVPlusPlus::Template).void }
      # Write the +template+ to an Excel file
      #
      # @param template [Template] The template to write
      def write(template)
        ::CSVPlusPlus::Writer::RubyXLBuilder.new(
          options: @options,
          position: @position,
          rows: template.rows
        ).build_workbook.write(@options.output_filename)
      end

      sig { override.void }
      # Write a backup of the current spreadsheet.
      def write_backup
        backup_file(@options)
      end
    end
  end
end

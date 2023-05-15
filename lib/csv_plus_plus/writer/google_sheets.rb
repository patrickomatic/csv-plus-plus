# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # A class that can write a +Template+ to Google Sheets (via their API)
    class GoogleSheets < ::CSVPlusPlus::Writer::Writer
      extend ::T::Sig
      include ::CSVPlusPlus::GoogleApiClient

      sig do
        params(options: ::CSVPlusPlus::Options::GoogleSheetsOptions, position: ::CSVPlusPlus::Runtime::Position).void
      end
      # @param options [Options::GoogleSheetsOptions]
      # @param position [Runtime::Position]
      def initialize(options, position)
        super(position)

        @options = ::T.let(options, ::CSVPlusPlus::Options::GoogleSheetsOptions)
        @reader = ::T.let(::CSVPlusPlus::Reader::GoogleSheets.new(options), ::CSVPlusPlus::Reader::GoogleSheets)
      end

      sig { override.params(template: ::CSVPlusPlus::Template).void }
      # Write a +template+ to Google Sheets
      #
      # @param template [Template]
      def write(template)
        create_sheet! if @options.create_if_not_exists

        update_cells!(template)
      end

      sig { override.void }
      # Write a backup of the Google Sheet that is about to be written
      def write_backup
        drive_client.copy_file(@options.sheet_id)
      end

      private

      sig { void }
      def create_sheet!
        return if @reader.sheet

        sheets_client.create_spreadsheet(@options.sheet_name)
      end

      sig { params(template: ::CSVPlusPlus::Template).void }
      def update_cells!(template)
        sheets_client.batch_update_spreadsheet(@options.sheet_id, builder(template).batch_update_spreadsheet_request)
      end

      sig { params(template: ::CSVPlusPlus::Template).returns(::CSVPlusPlus::Writer::GoogleSheetsBuilder) }
      def builder(template)
        ::CSVPlusPlus::Writer::GoogleSheetsBuilder.new(
          options: @options,
          position: @position,
          reader: @reader,
          rows: template.rows
        )
      end
    end
  end
end

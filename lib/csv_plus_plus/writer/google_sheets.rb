# typed: strict
# frozen_string_literal: true

require_relative '../google_api_client'
require_relative 'base_writer'
require_relative 'google_sheet_builder'

module CSVPlusPlus
  module Writer
    # A class that can write a +Template+ to Google Sheets (via their API)
    # rubocop:disable Metrics/ClassLength
    class GoogleSheets < ::CSVPlusPlus::Writer::BaseWriter
      extend ::T::Sig

      sig { returns(::String) }
      attr_reader :sheet_id

      sig { returns(::T.nilable(::String)) }
      attr_reader :sheet_name

      # TODO: it would be nice to raise this but we shouldn't expand out more than necessary for our data
      SPREADSHEET_INFINITY = 1000
      public_constant :SPREADSHEET_INFINITY

      sig { params(options: ::CSVPlusPlus::Options, position: ::CSVPlusPlus::Runtime::Position).void }
      # @param options [Options]
      # @param position [Runtime::Position]
      def initialize(options, position)
        super(options, position)

        # @current_values = ::T.let(nil, ::T.nilable(::T::Array
        @sheet_id = ::T.let(::T.must(options.google).sheet_id, ::String)
        @sheet_name = ::T.let(options.sheet_name, ::T.nilable(::String))
        @sheets_client = ::T.let(::CSVPlusPlus::GoogleApiClient.sheets_client, ::Google::Apis::SheetsV4::SheetsService)
      end

      sig { override.params(template: ::CSVPlusPlus::Template).void }
      # write a +template+ to Google Sheets
      #
      # @param template [Template]
      def write(template)
        fetch_spreadsheet!
        fetch_spreadsheet_values!

        create_sheet! if @options.create_if_not_exists

        update_cells!(template)
      end

      sig { override.params(_options: ::CSVPlusPlus::Options).void }
      # write a backup of the google sheet
      def write_backup(_options)
        drive_client = ::CSVPlusPlus::GoogleApiClient.drive_client
        drive_client.copy_file(@sheet_id)
      end

      private

      sig do
        params(
          formatted_values: ::Google::Apis::SheetsV4::ValueRange,
          formula_values: ::Google::Apis::SheetsV4::ValueRange
        ).returns(::T::Array[::T::Array[::T.nilable(::String)]])
      end
      def extract_current_values(formatted_values, formula_values)
        formatted_values.values.map.each_with_index do |row, x|
          row.map.each_with_index do |_cell, y|
            formula_value = formula_values.values[x][y]
            if formula_value.is_a?(::String) && formula_value.start_with?('=')
              formula_value
            else
              strip_to_nil(formatted_values.values[x][y])
            end
          end
        end
      end

      sig { void }
      # rubocop:disable Metrics/MethodLength
      def fetch_spreadsheet_values!
        formatted_values = get_all_spreadsheet_values('FORMATTED_VALUE')
        formula_values = get_all_spreadsheet_values('FORMULA')

        @current_values =
          ::T.let(
            if formula_values.values.nil? || formatted_values.values.nil?
              []
            else
              extract_current_values(formatted_values, formula_values)
            end,
            ::T.nilable(::T::Array[::T::Array[::T.nilable(::String)]])
          )
      end
      # rubocop:enable Metrics/MethodLength

      sig { params(range: ::String).returns(::String) }
      def format_range(range)
        @sheet_name ? "'#{@sheet_name}'!#{range}" : range
      end

      sig { returns(::String) }
      def full_range
        format_range('A1:Z1000')
      end

      sig { params(str: ::String).returns(::T.nilable(::String)) }
      def strip_to_nil(str)
        str.strip.empty? ? nil : str
      end

      sig { params(render_option: ::String).returns(::Google::Apis::SheetsV4::ValueRange) }
      def get_all_spreadsheet_values(render_option)
        @sheets_client.get_spreadsheet_values(@sheet_id, full_range, value_render_option: render_option)
      end

      sig { returns(::T.nilable(::Google::Apis::SheetsV4::Sheet)) }
      def sheet
        return unless @sheet_name

        spreadsheet.sheets.find { |s| s.properties.title.strip == @sheet_name.strip }
      end

      sig { returns(::Google::Apis::SheetsV4::Spreadsheet) }
      def spreadsheet
        @spreadsheet ||= ::T.let(
          @sheets_client.get_spreadsheet(@sheet_id),
          ::T.nilable(::Google::Apis::SheetsV4::Spreadsheet)
        )

        raise(::CSVPlusPlus::Error::WriterError, 'Unable to connect to google spreadsheet') unless @spreadsheet

        @spreadsheet
      end

      sig { void }
      def fetch_spreadsheet!
        return unless @sheet_name.nil?

        @sheet_name = spreadsheet.sheets&.first&.properties&.title
      end

      sig { void }
      def create_sheet!
        return if sheet

        @sheets_client.create_spreadsheet(@sheet_name)
        fetch_spreadsheet!
        @sheet_name = spreadsheet.sheets.last.properties.title
      end

      sig { params(template: ::CSVPlusPlus::Template).void }
      def update_cells!(template)
        @sheets_client.batch_update_spreadsheet(@sheet_id, builder(template).batch_update_spreadsheet_request)
      end

      sig { params(template: ::CSVPlusPlus::Template).returns(::CSVPlusPlus::Writer::GoogleSheetBuilder) }
      def builder(template)
        ::CSVPlusPlus::Writer::GoogleSheetBuilder.new(
          position: @position,
          rows: template.rows,
          sheet_id: sheet&.properties&.sheet_id,
          column_index: @options.offset[1],
          row_index: @options.offset[0],
          current_sheet_values: ::T.must(@current_values)
        )
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end

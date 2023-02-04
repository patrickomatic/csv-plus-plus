# frozen_string_literal: true

require_relative 'base_writer'
require_relative 'google_sheet_builder'

AUTH_SCOPES = ['https://www.googleapis.com/auth/spreadsheets'].freeze
FULL_RANGE = 'A1:Z1000'

module CSVPlusPlus
  module Writer
    # A class that can output a +Template+ to Google Sheets (via their API)
    class GoogleSheets < ::CSVPlusPlus::Writer::BaseWriter
      # XXX it would be nice to raise this but we shouldn't expand out more than necessary for our data
      SPREADSHEET_INFINITY = 1000
      public_constant :SPREADSHEET_INFINITY

      # initialize
      def initialize(options)
        super(options)

        @sheet_id = options.google.sheet_id
        @sheet_name = options.sheet_name
      end

      # write a +template+ to Google Sheets
      def write(template)
        auth!

        save_spreadsheet!
        save_spreadsheet_values!

        create_sheet! if @options.create_if_not_exists

        update_cells!(template)
      rescue ::Google::Apis::ClientError => e
        handle_google_error(e)
      end

      protected

      def load_requires
        require('google/apis/sheets_v4')
        require('googleauth')
      end

      private

      def format_range(range)
        @sheet_name ? "'#{@sheet_name}'!#{range}" : range
      end

      def full_range
        format_range(::FULL_RANGE)
      end

      def auth!
        @gs ||= sheets_ns::SheetsService.new
        @gs.authorization = ::Google::Auth.get_application_default(::AUTH_SCOPES)
      end

      def save_spreadsheet_values!
        formatted_values = get_all_spreadsheet_values('FORMATTED_VALUE')
        formula_values = get_all_spreadsheet_values('FORMULA')

        return if formula_values.values.nil? || formatted_values.values.nil?

        @current_values = extract_current_values(formatted_values, formula_values)
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

      def strip_to_nil(str)
        str.strip.empty? ? nil : str
      end

      def get_all_spreadsheet_values(render_option)
        @gs.get_spreadsheet_values(@sheet_id, full_range, value_render_option: render_option)
      end

      def sheet
        return unless @sheet_name

        @spreadsheet.sheets.find { |s| s.properties.title.strip == @sheet_name.strip }
      end

      def save_spreadsheet!
        @spreadsheet = @gs.get_spreadsheet(@sheet_id)

        return unless @sheet_name.nil?

        @sheet_name = @spreadsheet.sheets&.first&.properties&.title
      end

      def create_sheet!
        return if sheet

        @gs.create_spreadsheet(@sheet_name)
        get_spreadsheet!
        @sheet_name = @spreadsheet.sheets.last.properties.title
      end

      def update_cells!(template)
        builder = ::CSVPlusPlus::Writer::GoogleSheetBuilder.new(
          rows: template.rows,
          sheet_id: sheet&.properties&.sheet_id,
          column_index: @options.offset[1],
          row_index: @options.offset[0],
          current_sheet_values: @current_sheet_values
        )
        @gs.batch_update_spreadsheet(@sheet_id, builder.batch_update_spreadsheet_request)
      rescue ::Google::Apis::ClientError => e
        handle_google_error(e)
      end

      def sheets_ns
        ::Google::Apis::SheetsV4
      end

      def handle_google_error(error)
        if @options.verbose
          warn("#{error.status_code} Error making Google Sheets API request [#{error.message}]: #{error.body}")
        else
          warn("Error making Google Sheets API request: #{error.message}")
        end
      end
    end
  end
end

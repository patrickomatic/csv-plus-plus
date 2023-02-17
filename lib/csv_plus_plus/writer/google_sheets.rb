# frozen_string_literal: true

require_relative '../google_api_client'
require_relative 'base_writer'
require_relative 'google_sheet_builder'

module CSVPlusPlus
  module Writer
    # A class that can write a +Template+ to Google Sheets (via their API)
    class GoogleSheets < ::CSVPlusPlus::Writer::BaseWriter
      # TODO: it would be nice to raise this but we shouldn't expand out more than necessary for our data
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
        @sheets_client = ::CSVPlusPlus::GoogleApiClient.sheets_client

        fetch_spreadsheet!
        fetch_spreadsheet_values!

        create_sheet! if @options.create_if_not_exists

        update_cells!(template)
      end

      # write a backup of the google sheet
      def write_backup
        drive_client = ::CSVPlusPlus::GoogleApiClient.drive_client
        drive_client.copy_file(@sheet_id)
      end

      protected

      def load_requires
        require('google/apis/drive_v3')
        require('google/apis/sheets_v4')
        require('googleauth')
      end

      private

      def format_range(range)
        @sheet_name ? "'#{@sheet_name}'!#{range}" : range
      end

      def full_range
        format_range('A1:Z1000')
      end

      def fetch_spreadsheet_values!
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
        @sheets_client.get_spreadsheet_values(@sheet_id, full_range, value_render_option: render_option)
      end

      def sheet
        return unless @sheet_name

        @spreadsheet.sheets.find { |s| s.properties.title.strip == @sheet_name.strip }
      end

      def fetch_spreadsheet!
        @spreadsheet = @sheets_client.get_spreadsheet(@sheet_id)

        return unless @sheet_name.nil?

        @sheet_name = @spreadsheet.sheets&.first&.properties&.title
      end

      def create_sheet!
        return if sheet

        @sheets_client.create_spreadsheet(@sheet_name)
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
        @sheets_client.batch_update_spreadsheet(@sheet_id, builder.batch_update_spreadsheet_request)
      end
    end
  end
end

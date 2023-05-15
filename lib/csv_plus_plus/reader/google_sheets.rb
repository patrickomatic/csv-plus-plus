# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Reader
    # A class that can read an existing Google Sheets spreadsheet
    class GoogleSheets < ::CSVPlusPlus::Reader::Reader
      extend ::T::Sig
      extend ::T::Generic
      include ::CSVPlusPlus::GoogleApiClient

      CellValue = type_member { { fixed: ::String } }
      public_constant :CellValue

      sig { params(options: ::CSVPlusPlus::Options::GoogleSheetsOptions).void }
      # Open a CSV outputter to the +output_filename+ specified by the +Options+
      #
      # @param options [Options::GoogleSheetsOptions] The supplied options.
      def initialize(options)
        super()

        @options = options
        @cell_values = ::T.let(nil, ::T.nilable(::T::Array[::T::Array[::T.nilable(::String)]]))
      end

      sig do
        override.params(cell: ::CSVPlusPlus::Cell).returns(::T.nilable(::CSVPlusPlus::Reader::GoogleSheets::CellValue))
      end
      # Get the current value at the +cell+'s location.
      #
      # @param cell [Cell]
      #
      # @return [CellValue, nil]
      def value_at(cell)
        cell_values[cell.row_index]&.[](cell.index)
      end

      sig { returns(::T.nilable(::Google::Apis::SheetsV4::Sheet)) }
      # @return [Google::Apis::SheetsV4::Sheet, nil]
      def sheet
        spreadsheet.sheets.find { |s| s.properties.title.strip == @options.sheet_name.strip }
      end

      sig { returns(::Google::Apis::SheetsV4::Spreadsheet) }
      # @return [Google::Apis::SheetsV4::Spreadsheet]
      def spreadsheet
        @spreadsheet ||= ::T.let(
          sheets_client.get_spreadsheet(@options.sheet_id),
          ::T.nilable(::Google::Apis::SheetsV4::Spreadsheet)
        )

        unless @spreadsheet
          raise(::CSVPlusPlus::Error::WriterError, "Unable to connect to google spreadsheet #{@options.sheet_id}")
        end

        @spreadsheet
      end

      private

      sig { returns(::T::Array[::T::Array[::T.nilable(::String)]]) }
      # rubocop:disable Metrics/MethodLength
      def cell_values
        return @cell_values if @cell_values

        formatted_values = get_all_spreadsheet_values('FORMATTED_VALUE')
        formula_values = get_all_spreadsheet_values('FORMULA')

        @cell_values =
          ::T.must(
            ::T.let(
              if formula_values&.values.nil? || formatted_values&.values.nil?
                []
              else
                extract_current_values(::T.must(formatted_values), ::T.must(formula_values))
              end,
              ::T.nilable(::T::Array[::T::Array[::T.nilable(::String)]])
            )
          )
      end
      # rubocop:enable Metrics/MethodLength

      sig { params(render_option: ::String).returns(::T.nilable(::Google::Apis::SheetsV4::ValueRange)) }
      def get_all_spreadsheet_values(render_option)
        sheets_client.get_spreadsheet_values(@options.sheet_id, full_range, value_render_option: render_option)
      rescue ::Google::Apis::ClientError => e
        return if e.status_code == 404

        raise
      end

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

      sig { params(range: ::String).returns(::String) }
      def format_range(range)
        # "'#{@options.sheet_name}'!#{range}"
        # XXX
        range
      end

      sig { returns(::String) }
      def full_range
        format_range('A1:Z1000')
      end

      sig { params(str: ::String).returns(::T.nilable(::String)) }
      def strip_to_nil(str)
        str.strip.empty? ? nil : str
      end
    end
  end
end

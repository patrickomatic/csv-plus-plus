# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Given +rows+ from a +Template+, build requests compatible with Google Sheets Ruby API
    # rubocop:disable Metrics/ClassLength
    class GoogleSheetBuilder
      extend ::T::Sig

      sig do
        params(
          current_sheet_values: ::T::Array[::T::Array[::T.nilable(::String)]],
          position: ::CSVPlusPlus::Runtime::Position,
          sheet_id: ::T.nilable(::Integer),
          rows: ::T::Array[::CSVPlusPlus::Row],
          column_index: ::T.nilable(::Integer),
          row_index: ::T.nilable(::Integer)
        ).void
      end
      # @param column_index [Integer] Offset the results by +column_index+
      # @param current_sheet_values [Array<Array<::String, nil>>]
      # @param sheet_id [::String] The sheet ID referencing the sheet in Google
      # @param row_index [Integer] Offset the results by +row_index+
      # @param rows [Array<Row>] The rows to render
      # @param position [Position] The current position.
      #
      # rubocop:disable Metrics/ParameterLists
      def initialize(current_sheet_values:, position:, sheet_id:, rows:, column_index: 0, row_index: 0)
        # rubocop:enable Metrics/ParameterLists
        @current_sheet_values = current_sheet_values
        @sheet_id = sheet_id
        @rows = rows
        @column_index = column_index
        @row_index = row_index
        @position = position
      end

      sig { returns(::Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest) }
      # Build a Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest
      #
      # @return [Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest]
      def batch_update_spreadsheet_request
        build_batch_request(@rows)
      end

      private

      sig { params(extended_value: ::Google::Apis::SheetsV4::ExtendedValue, value: ::T.nilable(::String)).void }
      def set_extended_value_type!(extended_value, value)
        v = value || ''
        if v.start_with?('=')
          extended_value.formula_value = value
        elsif v.match(/^-?[\d.]+$/)
          extended_value.number_value = value
        elsif v.downcase == 'true' || v.downcase == 'false'
          extended_value.bool_value = value
        else
          extended_value.string_value = value
        end
      end

      sig { params(mod: ::CSVPlusPlus::Modifier::GoogleSheetModifier).returns(::Google::Apis::SheetsV4::CellFormat) }
      def build_cell_format(mod)
        ::Google::Apis::SheetsV4::CellFormat.new.tap do |cf|
          cf.text_format = mod.text_format

          cf.horizontal_alignment = mod.horizontal_alignment
          cf.vertical_alignment = mod.vertical_alignment
          cf.background_color = mod.background_color
          cf.number_format = mod.number_format
        end
      end

      sig { params(cell: ::CSVPlusPlus::Cell).returns(::Google::Apis::SheetsV4::GridRange) }
      def grid_range_for_cell(cell)
        ::Google::Apis::SheetsV4::GridRange.new(
          sheet_id: @sheet_id,
          start_column_index: cell.index,
          end_column_index: cell.index + 1,
          start_row_index: cell.row_index,
          end_row_index: cell.row_index + 1
        )
      end

      sig { params(row_index: ::Integer, cell_index: ::Integer).returns(::T.nilable(::String)) }
      def current_value(row_index, cell_index)
        ::T.must(@current_sheet_values[row_index])[cell_index]
      rescue ::StandardError
        nil
      end

      sig { params(cell: ::CSVPlusPlus::Cell).returns(::Google::Apis::SheetsV4::ExtendedValue) }
      def build_cell_value(cell)
        ::Google::Apis::SheetsV4::ExtendedValue.new.tap do |xv|
          value =
            if cell.value.nil?
              current_value(cell.row_index, cell.index)
            else
              cell.evaluate(@position)
            end

          set_extended_value_type!(xv, value)
        end
      end

      sig { params(cell: ::CSVPlusPlus::Cell).returns(::Google::Apis::SheetsV4::CellData) }
      def build_cell_data(cell)
        mod = cell.modifier

        ::Google::Apis::SheetsV4::CellData.new.tap do |cd|
          cd.user_entered_format = build_cell_format(::T.cast(mod, ::CSVPlusPlus::Modifier::GoogleSheetModifier))
          cd.note = mod.note if mod.note

          # TODO: apply data validation
          cd.user_entered_value = build_cell_value(cell)
        end
      end

      sig { params(row: ::CSVPlusPlus::Row).returns(::Google::Apis::SheetsV4::RowData) }
      def build_row_data(row)
        ::Google::Apis::SheetsV4::RowData.new(values: row.cells.map { |cell| build_cell_data(cell) })
      end

      sig { params(rows: ::T::Array[::CSVPlusPlus::Row]).returns(::Google::Apis::SheetsV4::UpdateCellsRequest) }
      def build_update_cells_request(rows)
        ::Google::Apis::SheetsV4::UpdateCellsRequest.new(
          fields: '*',
          start: ::Google::Apis::SheetsV4::GridCoordinate.new(
            sheet_id: @sheet_id,
            column_index: @column_index,
            row_index: @row_index
          ),
          rows:
        )
      end

      sig { params(cell: ::CSVPlusPlus::Cell).returns(::Google::Apis::SheetsV4::UpdateBordersRequest) }
      def build_border(cell)
        mod = ::T.cast(cell.modifier, ::CSVPlusPlus::Modifier::GoogleSheetModifier)
        border = mod.border

        ::Google::Apis::SheetsV4::UpdateBordersRequest.new(
          top: mod.border_along?(::CSVPlusPlus::Modifier::BorderSide::Top) ? border : nil,
          right: mod.border_along?(::CSVPlusPlus::Modifier::BorderSide::Right) ? border : nil,
          left: mod.border_along?(::CSVPlusPlus::Modifier::BorderSide::Left) ? border : nil,
          bottom: mod.border_along?(::CSVPlusPlus::Modifier::BorderSide::Bottom) ? border : nil,
          range: grid_range_for_cell(cell)
        )
      end

      sig { params(cell: ::CSVPlusPlus::Cell).returns(::Google::Apis::SheetsV4::Request) }
      def build_update_borders_request(cell)
        ::Google::Apis::SheetsV4::Request.new(update_borders: build_border(cell))
      end

      sig { params(rows: ::T::Array[::CSVPlusPlus::Row]).returns(::T::Array[::Google::Apis::SheetsV4::Request]) }
      # rubocop:disable Metrics/MethodLength
      def chunked_requests(rows)
        accum = []
        [].tap do |chunked|
          @position.map_rows(rows) do |row|
            accum << build_row_data(row)
            next unless accum.length == 1000

            chunked << ::Google::Apis::SheetsV4::Request.new(update_cells: build_update_cells_request(accum))
            accum = []
          end

          unless accum.empty?
            chunked << ::Google::Apis::SheetsV4::Request.new(update_cells: build_update_cells_request(accum))
          end
        end
      end
      # rubocop:enable Metrics/MethodLength

      sig do
        params(rows: ::T::Array[::CSVPlusPlus::Row]).returns(::Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest)
      end
      def build_batch_request(rows)
        ::Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest.new.tap do |bu|
          bu.requests = chunked_requests(rows)

          @position.map_rows(rows) do |row|
            row.cells.filter { |c| c.modifier.any_border? }
               .each do |cell|
                 bu.requests << build_update_borders_request(cell)
               end
          end
        end
      end
    end
    # rubocop:enable Metrics/ClassLength
  end
end

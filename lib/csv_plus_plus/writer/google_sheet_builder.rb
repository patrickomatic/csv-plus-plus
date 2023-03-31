# typed: false
# frozen_string_literal: true

require_relative './google_sheet_modifier'

module CSVPlusPlus
  module Writer
    # Given +rows+ from a +Template+, build requests compatible with Google Sheets Ruby API
    # rubocop:disable Metrics/ClassLength
    class GoogleSheetBuilder
      # @param column_index [Integer] Offset the results by +column_index+
      # @param current_sheet_values
      # @param sheet_id [String] The sheet ID referencing the sheet in Google
      # @param row_index [Integer] Offset the results by +row_index+
      # @param rows [Array<Row>] The rows to render
      # @param runtime [Runtime] The current runtime.
      #
      # rubocop:disable Metrics/ParameterLists
      def initialize(current_sheet_values:, runtime:, sheet_id:, rows:, column_index: 0, row_index: 0)
        # rubocop:enable Metrics/ParameterLists
        @current_sheet_values = current_sheet_values
        @sheet_id = sheet_id
        @rows = rows
        @column_index = column_index
        @row_index = row_index
        @runtime = runtime
      end

      # Build a Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest
      #
      # @return [Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest]
      def batch_update_spreadsheet_request
        build_batch_request(@rows)
      end

      private

      def set_extended_value_type!(extended_value, value)
        v = value || ''
        if v.start_with?('=')
          extended_value.formula_value = value
        elsif v.match(/^-?[\d.]+$/)
          extended_value.number_value = value
        elsif v.downcase == 'true' || v.downcase == 'false'
          extended_value.boolean_value = value
        else
          extended_value.string_value = value
        end
      end

      def build_cell_format(mod)
        ::Google::Apis::SheetsV4::CellFormat.new.tap do |cf|
          cf.text_format = mod.text_format

          cf.horizontal_alignment = mod.halign
          cf.vertical_alignment = mod.valign
          cf.background_color = mod.color
          cf.number_format = mod.numberformat
        end
      end

      def grid_range_for_cell(cell)
        ::Google::Apis::SheetsV4::GridRange.new(
          sheet_id: @sheet_id,
          start_column_index: cell.index,
          end_column_index: cell.index + 1,
          start_row_index: cell.row_index,
          end_row_index: cell.row_index + 1
        )
      end

      def current_value(row_index, cell_index)
        @current_sheet_values[row_index][cell_index]
      rescue ::StandardError
        nil
      end

      def build_cell_value(cell)
        ::Google::Apis::SheetsV4::ExtendedValue.new.tap do |xv|
          value =
            if cell.value.nil?
              current_value(cell.row_index, cell.index)
            else
              cell.evaluate(@runtime)
            end

          set_extended_value_type!(xv, value)
        end
      end

      def build_cell_data(cell)
        mod = ::CSVPlusPlus::Writer::GoogleSheetModifier.new(cell.modifier)

        ::Google::Apis::SheetsV4::CellData.new.tap do |cd|
          cd.user_entered_format = build_cell_format(mod)
          cd.note = mod.note if mod.note

          # XXX apply data validation
          cd.user_entered_value = build_cell_value(cell)
        end
      end

      def build_row_data(row)
        ::Google::Apis::SheetsV4::RowData.new(values: row.cells.map { |cell| build_cell_data(cell) })
      end

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

      def build_border(cell)
        mod = ::CSVPlusPlus::Writer::GoogleSheetModifier.new(cell.modifier)
        border = mod.border

        ::Google::Apis::SheetsV4::UpdateBordersRequest.new(
          top: mod.border_along?(:top) ? border : nil,
          right: mod.border_along?(:right) ? border : nil,
          left: mod.border_along?(:left) ? border : nil,
          bottom: mod.border_along?(:bottom) ? border : nil,
          range: grid_range_for_cell(cell)
        )
      end

      def build_update_borders_request(cell)
        ::Google::Apis::SheetsV4::Request.new(update_borders: build_border(cell))
      end

      # rubocop:disable Metrics/MethodLength
      def chunked_requests(rows)
        accum = []
        [].tap do |chunked|
          @runtime.map_rows(rows) do |row|
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

      def build_batch_request(rows)
        ::Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest.new.tap do |bu|
          bu.requests = chunked_requests(rows)

          @runtime.map_rows(rows) do |row|
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

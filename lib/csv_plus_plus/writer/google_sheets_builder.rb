# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Given +rows+ from a +Template+, build requests compatible with Google Sheets Ruby API
    # rubocop:disable Metrics/ClassLength
    class GoogleSheetsBuilder
      extend ::T::Sig
      include ::CSVPlusPlus::Writer::Merger

      sig do
        params(
          options: ::CSVPlusPlus::Options::GoogleSheetsOptions,
          position: ::CSVPlusPlus::Runtime::Position,
          reader: ::CSVPlusPlus::Reader::GoogleSheets,
          rows: ::T::Array[::CSVPlusPlus::Row]
        ).void
      end
      # @param options [Options]
      # @param position [Position] The current position.
      # @param reader [::CSVPlusPlus::Reader::GoogleSheets]
      # @param rows [Array<Row>] The rows to render
      def initialize(options:, position:, reader:, rows:)
        @options = options
        @position = position
        @reader = reader
        @rows = rows
      end

      sig { returns(::Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest) }
      # Build a Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest
      #
      # @return [Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest]
      def batch_update_spreadsheet_request
        build_batch_request(@rows)
      end

      private

      sig { params(value: ::T.nilable(::String)).returns(::Google::Apis::SheetsV4::ExtendedValue) }
      # rubocop:disable Metrics/MethodLength
      def build_extended_value(value)
        ::Google::Apis::SheetsV4::ExtendedValue.new.tap do |xv|
          v = value || ''
          if v.start_with?('=')
            xv.formula_value = value
          elsif v.match(/^-?[\d.]+$/)
            xv.number_value = value
          elsif v.downcase == 'true' || v.downcase == 'false'
            xv.bool_value = value
          else
            xv.string_value = value
          end
        end
      end
      # rubocop:enable Metrics/MethodLength

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
          start_column_index: cell.index,
          end_column_index: cell.index + 1,
          start_row_index: cell.row_index,
          end_row_index: cell.row_index + 1
        )
      end

      sig { params(cell: ::CSVPlusPlus::Cell).returns(::Google::Apis::SheetsV4::ExtendedValue) }
      def build_cell_value(cell)
        build_extended_value(
          merge_cell_value(
            existing_value: @reader.value_at(cell),
            new_value: (ast = cell.ast) ? ast.evaluate(@position) : cell.value,
            options: @options
          )
        )
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
            column_index: @options.offset[1],
            row_index: @options.offset[0]
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

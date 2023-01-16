# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    ##
    # Given +rows+ from a +Template+, build requests compatible with Google Sheets Ruby API
    # rubocop:disable Metrics/ClassLength
    class GoogleSheetBuilder
      # initialize
      def initialize(current_sheet_values:, sheet_id:, rows:, column_index: 0, row_index: 0)
        @current_sheet_values = current_sheet_values
        @sheet_id = sheet_id
        @rows = rows
        @column_index = column_index
        @row_index = row_index
      end

      # Build a Google::Apis::SheetsV4::BatchUpdateSpreadsheetRequest
      def batch_update_spreadsheet_request
        build_batch_request(@rows)
      end

      private

      def sheets_ns
        ::Google::Apis::SheetsV4
      end

      def sheets_color(color)
        sheets_ns::Color.new(red: color.red, green: color.green, blue: color.blue)
      end

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

      # rubocop:disable Metrics/AbcSize, Metrics/CyclomaticComplexity
      def build_text_format(mod)
        sheets_ns::TextFormat.new.tap do |tf|
          tf.bold = true if mod.formatted?('bold')
          tf.italic = true if mod.formatted?('italic')
          tf.strikethrough = true if mod.formatted?('strikethrough')
          tf.underline = true if mod.formatted?('underline')

          tf.font_family = mod.fontfamily if mod.fontfamily
          tf.font_size = mod.fontsize if mod.fontsize

          tf.foreground_color = sheets_color(mod.fontcolor) if mod.fontcolor
        end
      end
      # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity

      # rubocop:disable Metrics/AbcSize, Metrics/CyclomaticComplexity
      def build_cell_format(mod)
        sheets_ns::CellFormat.new.tap do |cf|
          cf.text_format = build_text_format(mod)

          # TODO: are these not overwriting each other?
          cf.horizontal_alignment = 'LEFT' if mod.aligned?('left')
          cf.horizontal_alignment = 'RIGHT' if mod.aligned?('right')
          cf.horizontal_alignment = 'CENTER' if mod.aligned?('center')
          cf.vertical_alignment = 'TOP' if mod.aligned?('top')
          cf.vertical_alignment = 'BOTTOM' if mod.aligned?('bottom')

          cf.background_color = sheets_color(mod.color) if mod.color

          cf.number_format = sheets_ns::NumberFormat.new(type: mod.numberformat) if mod.numberformat
        end
      end
      # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity

      def grid_range_for_cell(cell)
        sheets_ns::GridRange.new(
          sheet_id: @sheet_id,
          start_column_index: cell.index,
          end_column_index: cell.index + 1,
          start_row_index: cell.row_index,
          end_row_index: cell.row_index + 1
        )
      end

      def current_value(row_index, cell_index)
        @current_values[row_index][cell_index]
      rescue ::StandardError
        nil
      end

      def build_cell_value(cell)
        sheets_ns::ExtendedValue.new.tap do |xv|
          value =
            if cell.value.nil?
              current_value(cell.row_index, cell.index)
            else
              cell.to_csv
            end

          set_extended_value_type!(xv, value)
        end
      end

      def build_cell_data(cell)
        mod = cell.modifier

        sheets_ns::CellData.new.tap do |cd|
          cd.user_entered_format = build_cell_format(cell.modifier)
          cd.note = mod.note if mod.note

          # XXX apply data validation
          cd.user_entered_value = build_cell_value(cell)
        end
      end

      def build_row_data(row)
        sheets_ns::RowData.new(values: row.cells.map { |cell| build_cell_data(cell) })
      end

      def build_update_cells_request(rows)
        sheets_ns::UpdateCellsRequest.new(
          fields: '*',
          start: sheets_ns::GridCoordinate.new(
            sheet_id: @sheet_id,
            column_index: @column_index,
            row_index: @row_index
          ),
          rows: rows.map { |row| build_row_data(row) }
        )
      end

      # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
      def build_update_borders_request(cell)
        mod = cell.modifier
        sheets_ns::Request.new(
          update_borders:
            sheets_ns::UpdateBordersRequest.new.tap do |br|
              # TODO: allow different border styles per side
              border = sheets_ns::Border.new(color: mod.bordercolor || '#000000', style: mod.borderstyle || 'solid')
              br.top = border if mod.border_along?('top')
              br.right = border if mod.border_along?('right')
              br.left = border if mod.border_along?('left')
              br.bottom = border if mod.border_along?('bottom')

              br.range = grid_range_for_cell(cell)
            end
        )
      end
      # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

      # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
      def build_batch_request(rows)
        sheets_ns::BatchUpdateSpreadsheetRequest.new.tap do |bu|
          bu.requests =
            rows.each_slice(1000).to_a.map do |chunked_rows|
              sheets_ns::Request.new(update_cells: build_update_cells_request(chunked_rows))
            end

          rows.each do |row|
            row.cells.filter { |c| c.modifier.any_border? }
               .each do |cell|
                 bu.requests << build_update_borders_request(cell)
               end
          end
        end
      end
      # rubocop:enable Metrics/AbcSize, Metrics/MethodLength
    end
    # rubocop:enable Metrics/ClassLength
  end
end

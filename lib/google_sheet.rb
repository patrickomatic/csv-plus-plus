# frozen_string_literal: true

require 'google/apis/sheets_v4'
require 'googleauth'

SheetsApi = ::Google::Apis::SheetsV4
AUTH_SCOPES = ['https://www.googleapis.com/auth/spreadsheets'].freeze
FULL_RANGE = 'A1:Z1000'

module CSVPlusPlus
  ##
  # A class which can output a Template to Google Spredsheets (via their API)
  # rubocop:disable Metrics/ClassLength
  class GoogleSheet
    # XXX it would be nice to raise this but we shouldn't expand out more than necessary for our data
    SPREADSHEET_INFINITY = 1000
    public_constant :SPREADSHEET_INFINITY

    attr_reader :sheet_id, :sheet_name

    # initialize
    def initialize(
      sheet_id,
      sheet_name: nil,
      cell_offset: 0,
      row_offset: 0,
      create_if_not_exists: false
    )
      @sheet_name = sheet_name
      @sheet_id = sheet_id
      @cell_offset = cell_offset
      @row_offset = row_offset
      @create_if_not_exists = create_if_not_exists
    end

    # Write the template to Google Sheets
    def push!(template)
      auth!

      save_spreadsheet!
      save_spreadsheet_values!

      create_sheet! if @create_if_not_exists

      update_cells!(template)
    end

    protected

    def format_range(range)
      @sheet_name ? "'#{@sheet_name}'!#{range}" : range
    end

    def full_range
      format_range(::FULL_RANGE)
    end

    def auth!
      @gs ||= ::SheetsApi::SheetsService.new
      @gs.authorization = ::Google::Auth.get_application_default(::AUTH_SCOPES)
    end

    # rubocop:disable Metrics/AbcSize, Metrics/PerceivedComplexity, Metrics/CyclomaticComplexity, Metrics/MethodLength
    def save_spreadsheet_values!
      formatted_values = @gs.get_spreadsheet_values(@sheet_id, full_range, value_render_option: 'FORMATTED_VALUE')
      formula_values = @gs.get_spreadsheet_values(@sheet_id, full_range, value_render_option: 'FORMULA')
      return if formula_values.values.nil? || formatted_values.values.nil?

      @current_values =
        formatted_values.values.map.each_with_index do |row, x|
          row.map.each_with_index do |_cell, y|
            formula_value = formula_values.values[x][y]
            if formula_value.is_a?(::String) && formula_value.start_with?('=')
              formula_value
            else
              formatted_value = formatted_values.values[x][y]
              formatted_value.strip.empty? ? nil : formatted_value
            end
          end
        end
    end
    # rubocop:enable Metrics/AbcSize, Metrics/PerceivedComplexity, Metrics/CyclomaticComplexity, Metrics/MethodLength

    def sheet
      return unless @sheet_name

      @spreadsheet.sheets.find { |s| s.properties.title.strip == @sheet_name.strip }
    end

    def save_spreadsheet!
      @spreadsheet = @gs.get_spreadsheet(@sheet_id)

      return unless @sheet_name.nil?

      @sheet_name = @spreadsheet.sheets.first.properties.title
    end

    def create_sheet!
      return if sheet

      @gs.create_spreadsheet(@sheet_name)
      get_spreadsheet!
      @sheet_name = @spreadsheet.sheets.last.properties.title
    end

    # rubocop:disable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/PerceivedComplexity, Metrics/MethodLength
    def build_text_format(mod)
      ::SheetsApi::CellFormat.new.tap do |cf|
        cf.text_format =
          ::SheetsApi::TextFormat.new.tap do |tf|
            tf.bold = true if mod.formatted?('bold')
            tf.italic = true if mod.formatted?('italic')
            tf.strikethrough = true if mod.formatted?('strikethrough')
            tf.underline = true if mod.formatted?('underline')

            tf.font_family = mod.fontfamily if mod.fontfamily
            tf.font_size = mod.fontsize if mod.fontsize

            tf.foreground_color = sheets_color(mod.fontcolor) if mod.fontcolor
          end

        cf.horizontal_alignment = 'LEFT' if mod.aligned?('left')
        cf.horizontal_alignment = 'RIGHT' if mod.aligned?('right')
        cf.horizontal_alignment = 'CENTER' if mod.aligned?('center')
        cf.vertical_alignment = 'TOP' if mod.aligned?('top')
        cf.vertical_alignment = 'BOTTOM' if mod.aligned?('bottom')

        cf.background_color = sheets_color(mod.color) if mod.color

        cf.number_format = ::SheetsApi::NumberFormat.new(type: mod.numberformat) if mod.numberformat
      end
    end
    # rubocop:enable Metrics/AbcSize, Metrics/CyclomaticComplexity, Metrics/PerceivedComplexity, Metrics/MethodLength

    # TODO: eventually we can probably have a mix-in and put some methods in Cell
    # or maybe make a GoogleSheetCell wrapper that has a Cell
    def grid_range_for_cell(cell)
      ::SheetsApi::GridRange.new(
        sheet_id: sheet.properties.sheet_id,
        start_column_index: cell.index,
        end_column_index: cell.index + 1,
        start_row_index: cell.row_index,
        end_row_index: cell.row_index + 1
      )
    end

    # rubocop:disable Metrics/MethodLength
    def build_cell_value(cell)
      ::SheetsApi::ExtendedValue.new.tap do |xv|
        value =
          if cell.value.nil?
            begin
              @current_values[cell.row_index][cell.index]
            rescue ::StandardError
              nil
            end
          else
            cell.to_csv
          end

        set_extended_value_type!(xv, value)
      end
    end
    # rubocop:enable Metrics/MethodLength

    def build_cell_data(cell)
      mod = cell.modifier

      ::SheetsApi::CellData.new.tap do |cd|
        cd.user_entered_format = build_text_format(cell.modifier)
        cd.note = mod.note if mod.note

        # XXX apply data validation
        cd.user_entered_value = build_cell_value(cell)
      end
    end

    def build_row_data(row)
      ::SheetsApi::RowData.new(values: row.cells.map { |cell| build_cell_data(cell) })
    end

    def build_update_cells_request(rows)
      ::SheetsApi::UpdateCellsRequest.new.tap do |uc|
        uc.fields = '*'
        uc.start = ::SheetsApi::GridCoordinate.new(
          sheet_id: sheet.properties.sheet_id,
          row_index: @row_offset,
          column_index: @cell_offset
        )
        uc.rows = rows.map { |row| build_row_data(row) }
      end
    end

    # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
    def build_update_borders_request(cell)
      mod = cell.modifier
      ::SheetsApi::Request.new.tap do |r|
        r.update_borders =
          ::SheetsApi::UpdateBordersRequest.new.tap do |br|
            # TODO: allow different border styles per side
            border = ::SheetsApi::Border.new(color: mod.bordercolor || '#000000', style: mod.borderstyle || 'solid')
            br.top = border if mod.border_along?('top')
            br.right = border if mod.border_along?('right')
            br.left = border if mod.border_along?('left')
            br.bottom = border if mod.border_along?('bottom')

            br.range = grid_range_for_cell(cell)
          end
      end
    end
    # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

    # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
    def update_cells!(template)
      batch_request =
        ::SheetsApi::BatchUpdateSpreadsheetRequest.new.tap do |bu|
          bu.requests =
            template.rows.each_slice(1000).to_a.map do |rows|
              ::SheetsApi::Request.new.tap do |r|
                r.update_cells = build_update_cells_request(rows)
              end
            end

          template.rows.each do |row|
            row.cells.filter { |c| c.modifier.any_border? }
               .each do |cell|
              bu.requests << build_update_borders_request(cell)
            end
          end
        end

      puts("Calling batch_update_spreadsheet on #{@sheet_id}/#{@sheet_name} with", batch_request) if @verbose

      @gs.batch_update_spreadsheet(@sheet_id, batch_request)
    end
    # rubocop:enable Metrics/AbcSize, Metrics/MethodLength

    private

    def sheets_color(color)
      ::SheetsApi::Color.new(red: color.red, green: color.green, blue: color.blue)
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
  end
  # rubocop:enable Metrics/ClassLength
end

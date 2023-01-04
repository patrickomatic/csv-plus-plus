require 'googleauth'
require 'google/apis/sheets_v4'

module CSVPlusPlus
  class GoogleSheet
    SPREADSHEET_AUTH_SCOPES = ["https://www.googleapis.com/auth/spreadsheets"]
    # XXX it would be nice to raise this but we shouldn't expand out more than necessary for our data
    SPREADSHEET_INFINITY = 1000
    FULL_RANGE = "A1:Z#{SPREADSHEET_INFINITY}"

    SheetsApi = Google::Apis::SheetsV4

    attr_reader :sheet_id, :sheet_name

    def initialize(sheet_id,
                   sheet_name: nil, verbose: false,
                   cell_offset: 0, row_offset: 0, create_if_not_exists: false)
      @sheet_name = sheet_name
      @sheet_id = sheet_id
      @verbose = verbose
      @cell_offset = cell_offset
      @row_offset = row_offset
    end

    def push!(template)
      auth!

      get_spreadsheet!
      get_spreadsheet_values!

      create_sheet! if @create_if_not_exists

      update_cells!(template)
    end

    protected

    def format_range range
      @sheet_name ? "'#{@sheet_name}'!#{range}" : range
    end

    def full_range
      format_range FULL_RANGE
    end

    def auth!
      @gs ||= SheetsApi::SheetsService.new
      @gs.authorization = Google::Auth.get_application_default(SPREADSHEET_AUTH_SCOPES)
    end

    def get_spreadsheet_values!
      formatted_values = @gs.get_spreadsheet_values(@sheet_id, full_range,
                                                    value_render_option: 'FORMATTED_VALUE')
      formula_values = @gs.get_spreadsheet_values(@sheet_id, full_range,
                                                  value_render_option: 'FORMULA')
      if formula_values.values.nil? || formatted_values.values.nil?
        return
      end

      @current_values = formatted_values.values.map.each_with_index do |row, x|
        row.map.each_with_index do |cell, y|
          formula_value = formula_values.values[x][y]
          if formula_value.is_a?(String) && formula_value.start_with?('=')
            formula_value
          else
            formatted_value = formatted_values.values[x][y]
            formatted_value.strip.empty? ? nil : formatted_value
          end
        end
      end
    end

    def sheet
      return nil unless @sheet_name
      @spreadsheet.sheets.find {|s| s.properties.title.strip == @sheet_name.strip}
    end

    def get_spreadsheet!
      @spreadsheet = @gs.get_spreadsheet(@sheet_id)

      if @sheet_name.nil?
        @sheet_name = @spreadsheet.sheets.first.properties.title
      end
    end

    def create_sheet!
      return if sheet

      @gs.create_spreadsheet(@sheet_name)
      get_spreadsheet!
      @sheet_name = @spreadsheet.sheets.last.properties.title
    end

    def build_text_format(mod)
      SheetsApi::CellFormat.new.tap do |cf| 
        cf.text_format = SheetsApi::TextFormat.new.tap do |tf|
          tf.bold = true if mod.bold?
          tf.italic = true if mod.italic?
          tf.strikethrough = true if mod.strikethrough?
          tf.underline = true if mod.underline?

          tf.font_family = mod.fontfamily if mod.fontfamily
          tf.font_size = mod.fontsize if mod.fontsize
          if mod.fontcolor
            tf.foreground_color = SheetsApi::Color.new(
              red: mod.fontcolor.red,
              green: mod.fontcolor.green,
              blue: mod.fontcolor.blue,
            )
          end
        end

        cf.horizontal_alignment = 'LEFT' if mod.left_align?
        cf.horizontal_alignment = 'RIGHT' if mod.right_align?
        cf.horizontal_alignment = 'CENTER' if mod.center_align?

        cf.vertical_alignment = 'TOP' if mod.top_align?
        cf.vertical_alignment = 'BOTTOM' if mod.bottom_align?

        if mod.color
          cf.background_color = SheetsApi::Color.new(
            red: mod.color.red,
            green: mod.color.green,
            blue: mod.color.blue,
          )
        end

        if mod.numberformat
          cf.number_format = SheetsApi::NumberFormat.new(type: mod.numberformat)
        end
      end
    end

    # TODO eventually we can probably have a mix-in and put some methods in Cell
    # or maybe make a GoogleSheetCell wrapper that has a Cell
    def grid_range_for_cell(cell)
      SheetsApi::GridRange.new(
        sheet_id: sheet.properties.sheet_id,
        start_column_index: cell.index,
        end_column_index: cell.index + 1,
        start_row_index: cell.row_index,
        end_row_index: cell.row_index + 1,
      )
    end

    def build_cell_value(cell)
      SheetsApi::ExtendedValue.new.tap do |xv|
        value = cell.value.nil? ?
          (@current_values[cell.row_index][cell.index] rescue nil) : 
          cell.to_csv

        set_extended_value_type!(xv, value)
      end
    end

    def build_cell_data(cell)
      mod = cell.modifier

      SheetsApi::CellData.new.tap do |cd|
        cd.user_entered_format = build_text_format(cell.modifier)
        cd.note = mod.note if mod.note 

        # XXX apply data validation
        cd.user_entered_value = build_cell_value(cell)
      end
    end

    def build_row_data(row) 
      SheetsApi::RowData.new(values: row.cells.map {|cell| build_cell_data(cell)})
    end

    def build_update_cells_request(rows)
      SheetsApi::UpdateCellsRequest.new.tap do |uc|
        uc.fields = '*'
        uc.start = SheetsApi::GridCoordinate.new(
          sheet_id: sheet.properties.sheet_id,
          row_index: @row_offset,
          column_index: @cell_offset,
        )
        uc.rows = rows.map {|row| build_row_data(row)}
      end
    end

    def build_update_borders_request(cell)
      mod = cell.modifier
      SheetsApi::Request.new.tap do |r|
        r.update_borders = SheetsApi::UpdateBordersRequest.new.tap do |br|
          # TODO allow different border styles per side
          border = SheetsApi::Border.new(
            color: mod.bordercolor || '#000000',
            style: mod.borderstyle || 'solid',
          )
          br.top = border if mod.border_top?
          br.right = border if mod.border_right?
          br.left = border if mod.border_left?
          br.bottom = border if mod.border_bottom?

          br.range = grid_range_for_cell cell
        end
      end
    end

    def update_cells!(template)
      batch_request = SheetsApi::BatchUpdateSpreadsheetRequest.new.tap do |bu|
        bu.requests = template.rows.each_slice(1000).to_a.map do |rows|
          SheetsApi::Request.new.tap do |r|
            r.update_cells = build_update_cells_request(rows)
          end
        end

        template.rows.each do |row|
          row.cells.filter {|c| c.modifier.has_border? }.each do |cell|
            bu.requests << build_update_borders_request(cell)
          end
        end
      end

      if @verbose
        puts "Calling batch_update_spreadsheet on #@sheet_id/#@sheet_name with", batch_request
      end

      @gs.batch_update_spreadsheet(@sheet_id, batch_request)
    end

    private

    def set_extended_value_type!(extended_value, value)
      if value.nil? 
        extended_value.string_value = value
      elsif value.start_with? '='
        extended_value.formula_value = value
      elsif value.match(/^-?[\d\.]+$/)
        extended_value.number_value = value
      elsif value.downcase == 'true' || value.downcase == 'false'
        extended_value.boolean_value = value
      else
        extended_value.string_value = value
      end
    end
  end
end

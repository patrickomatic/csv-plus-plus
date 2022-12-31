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

    def initialize(sheet_id, sheet_name: nil, verbose: false, cell_offset: 0, row_offset: 0)
      @sheet_name = sheet_name
      @sheet_id = sheet_id
      @verbose = verbose
      @cell_offset = cell_offset
      @row_offset = row_offset

      auth_with_gs!
    end

    def auth_with_gs!
      @gs ||= SheetsApi::SheetsService.new
      @gs.authorization = Google::Auth.get_application_default(SPREADSHEET_AUTH_SCOPES)
    end

    def get_current_values!
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

    def format_range range
      @sheet_name ? "'#{@sheet_name}'!#{range}" : range
    end

    def full_range
      format_range FULL_RANGE
    end

    def push!(template)
      get_current_values!
      update_cells!(template)
      #update_cell_values!(template)
    end

    private

    def extended_value_type(value)
      if value.nil? 
        return 'string'
      elsif value.start_with? '='
        return 'formula'
      elsif value.match(/^-?[\d\.]+$/)
        return 'number'
      elsif value.downcase == 'true' || value.downcase == 'false'
        return 'boolean'
      else
        return 'string'
      end
    end

    def update_cells!(template)
      batch_request = SheetsApi::BatchUpdateSpreadsheetRequest.new.tap do |bu|
        bu.requests = template.rows.each_slice(1000).to_a.map do |rows|
          SheetsApi::Request.new.tap do |r|
            r.update_cells = SheetsApi::UpdateCellsRequest.new.tap do |uc|
              uc.fields = '*'
              uc.start = SheetsApi::GridCoordinate.new.tap do |gc|
                # figure out how to query this
                gc.sheet_id = @sheet_name == "Sheet1" ? 0 : 1704582377
                gc.column_index = @cell_offset
                gc.row_index = @row_offset
              end

              uc.rows = rows.map.with_index do |row, row_index|
                SheetsApi::RowData.new.tap do |rd|
                  rd.values = row.cells.map.with_index do |cell, cell_index| 
                    mod = cell.modifier

                    SheetsApi::CellData.new.tap do |cd|
                      cd.user_entered_format = SheetsApi::CellFormat.new.tap do |cf| 
                        # XXX I'm not sure if these work (bold,etc)
                        cf.text_format = SheetsApi::TextFormat.new.tap do |tf|
                          tf.bold = true if mod&.bold?
                          tf.italic = true if mod&.italic? 
                          tf.underline = true if mod&.underline? 
                          tf.strikethrough = true if mod&.strikethrough? 
                          tf.font_family = mod.fontfamily if mod&.fontfamily
                          tf.foreground_color = mod.fontcolor if mod&.fontcolor
                          # TODO what's the difference with this one
                          # tf.foreground_color_style = cell.fontcolor if cell.fontcolor
                        end
                      end

                      cd.note = mod.note if mod&.note 
                      cd.hyperlink = mod.hyperlink if mod&.hyperlink
                      # XXX apply borders
                      # XXX apply data validation
                      cd.user_entered_value = SheetsApi::ExtendedValue.new.tap do |xv|
                        value = cell.value.nil? ? 
                            (@current_values[row_index][cell_index] rescue nil) : 
                            cell.to_csv

                        case extended_value_type(value)
                        when 'string'
                          xv.string_value = value
                        when 'boolean'
                          xv.boolean_value = value
                        when 'formula'
                          xv.formula_value = value
                        when 'number'
                          xv.number_value = value
                        end
                      end
                    end
                  end
                end
              end
            end
          end
        end
      end

      if @verbose
        puts "Calling batch_update_spreadsheet on #@sheet_id/#@sheet_name with", batch_request
      end

      @gs.batch_update_spreadsheet(@sheet_id, batch_request)
    end
  end
end

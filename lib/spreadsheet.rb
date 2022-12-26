require 'googleauth'
require 'google/apis/sheets_v4'

module GSPush
  class Spreadsheet 
    SPREADSHEET_AUTH_SCOPES = ["https://www.googleapis.com/auth/spreadsheets"]
    SPREADSHEET_INFINITY = 1000
    FULL_RANGE = "A1:Z#{SPREADSHEET_INFINITY}"

    SheetsApi = Google::Apis::SheetsV4

    def initialize(sheet_id, sheet_name, headers)
      @sheet_name = sheet_name
      @sheet_id = sheet_id
      @headers = headers

      auth_with_gs!
    end

    def auth_with_gs!
      @gs ||= SheetsApi::SheetsService.new
      @gs.authorization = Google::Auth.get_application_default(SPREADSHEET_AUTH_SCOPES)
    end

    def get_values(range)
      @gs.get_spreadsheet_values(@sheet_id, "#{@sheet_name}!#{range}")
    end

    def full_range
      "#{@sheet_name}!#{FULL_RANGE}"
    end

    def get_all_values
      get_values(FULL_RANGE)
    end

    def push!(rows)
      update_cell_formatting!(rows)
      update_cell_values!(rows)
    end

    private

    def update_cell_formatting!(rows)
      batch_request = SheetsApi::BatchUpdateSpreadsheetRequest.new.tap do |bu|
        bu.requests = [
          SheetsApi::Request.new.tap do |r|
            r.update_cells = SheetsApi::UpdateCellsRequest.new.tap do |uc|
              uc.fields = '*'
              uc.range = full_range
              uc.rows = rows.map do |row| 
                SheetsApi::RowData.new.tap do |rd|
                  rd.values = row.cells.map do |cell| 
                    SheetsApi::CellData.new.tap do |cd|
                      cd.user_entered_format = SheetsApi::CellFormat.new.tap do |cf| 
                        cf.text_format = SheetsApi::TextFormat.new.tap do |tf|
                          tf.bold = true if cell.bold?
                          tf.italic = true if cell.italic?
                        end
                      end
                      # TODO cd.note
                      # TODO cd.hyperlink
                    end
                  end
                end
              end
            end
          end
        ]
      end

      @gs.batch_update_spreadsheet(@sheet_id, batch_request)
    end

    def update_cell_values!(rows)
      request = SheetsApi::BatchUpdateValuesRequest.new.tap do |r|
        r.data = [
          SheetsApi::ValueRange.new.tap do |d|
            d.values = rows.map {|row| row.cells.map {|c| c.value}}
            d.major_dimension = "ROWS"
            d.range = "A1"
          end
        ]
        r.value_input_option = 'USER_ENTERED'
      end

      @gs.batch_update_values(@sheet_id, request)
    end
  end
end

require 'googleauth'
require 'google/apis/sheets_v4'

module GSPush
  class Spreadsheet 
    SPREADSHEET_SCOPES = ["https://www.googleapis.com/auth/spreadsheets"];
    FULL_RANGE = "A1:Z1000"

    def initialize(sheet_id, sheet_name, header)
      @sheet_name = sheet_name
      @sheet_id = sheet_id
      @headers = headers

      auth_with_gs!
    end

    def auth_with_gs!
      @gs ||= Google::Apis::SheetsV4::SheetsService.new
      @gs.authorization = Google::Auth.get_application_default(SPREADSHEET_SCOPES)
    end

    def get_all_values
      @gs.get_spreadsheet_values(@sheet_id, "#{@sheet_name}!#{FULL_RANGE}")
    end

    def update_all_values(values)
      data = Google::Apis::SheetsV4::ValueRange.new.tap {|d|
        d.values = values
        d.major_dimension = "ROWS"
        d.range = "A1"
      }

      request = Google::Apis::SheetsV4::BatchUpdateValuesRequest.new
      request.data = [data]
      request.value_input_option = 'USER_ENTERED'

      @gs.batch_update_values(@sheet_id, request)
    end
  end
end

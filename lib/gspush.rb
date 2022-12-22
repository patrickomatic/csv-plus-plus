require 'csv'
require 'googleauth'
require 'google/apis/sheets_v4'

module GSPush 
  SPREADSHEET_SCOPES = ["https://www.googleapis.com/auth/spreadsheets"];

#  class GoogleSheet
#    def initialize(
  
  class CSVTemplate
    def initialize(input, key_values)
      @csv = CSV.new(input)
      @key_values = key_values
    end
  end

  def self.apply_template_to_sheet!(template_input, sheet_id, sheet_name, header: false, offset: 0, cell_offset: 0, key_values: nil)
    template = CSVTemplate.new(template_input, key_values)
    spreadsheet = Google::Apis::SheetsV4::SheetsService.new
    spreadsheet.authorization = Google::Auth.get_application_default(SPREADSHEET_SCOPES)
    rows = spreadsheet.get_spreadsheet_values(sheet_id, "#{sheet_name}!A1:Z1000")

    pp rows
  end
end

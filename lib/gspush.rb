require_relative 'csv_template'
require_relative 'spreadsheet'

module GSPush 
  def self.apply_template_to_sheet!(template_input, sheet_id, sheet_name, headers: false, offset: 0, cell_offset: 0, key_values: nil)
    template = CSVTemplate.new(template_input, key_values, headers)
    spreadsheet = Spreadsheet.new(sheet_id, sheet_name, headers)

    spreadsheet.update_all_values(template.get_all_values)
  rescue Google::Apis::ClientError => e
    $stderr.puts "#{e.status_code} Error making Google API request [#{e.message}]: #{e.body}"
    exit 1
  end
end

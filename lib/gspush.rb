require_relative 'template'
require_relative 'spreadsheet'

module GSPush 
  # TODO handle offsets and header
  def self.apply_template_to_sheet!(template_input, sheet_id, sheet_name, 
                                    headers: false, offset: 0, cell_offset: 0, 
                                    key_values: nil, verbose: false)
    template = Template.new(template_input, key_values: key_values, verbose: verbose)
    template.process!

    spreadsheet = Spreadsheet.new(sheet_id, sheet_name, headers, verbose)
    spreadsheet.push!(template.get_all_values)
  rescue Google::Apis::ClientError => e
    if verbose
      $stderr.puts "#{e.status_code} Error making Google Sheets API request [#{e.message}]: #{e.body}"
    else
      $stderr.puts "Error making Google Sheets API request: #{e.message}"
    end

    exit 1
  end
end

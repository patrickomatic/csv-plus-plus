require_relative 'template'
require_relative 'spreadsheet'

module GSPush 
  # TODO handle offsets and header
  def self.apply_template_to_sheet!(template_input, sheet_id, sheet_name, 
                                    headers: false, offset: 0, cell_offset: 0, key_values: nil)
    template = Template.new(template_input, key_values: key_values)
    template.process!

    spreadsheet = Spreadsheet.new(sheet_id, sheet_name, headers)
    spreadsheet.push!(template.get_all_values)
  rescue Google::Apis::ClientError => e
    # TODO only include body in verbose mode 
    $stderr.puts "#{e.status_code} Error making Google API request [#{e.message}]: #{e.body}"
    exit 1
  end
end

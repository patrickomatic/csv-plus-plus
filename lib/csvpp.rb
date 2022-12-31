require_relative 'template'
require_relative 'google_sheet'

module CSVPlusPlus
  def self.apply_template_to_sheet!(template_input, sheet_id, 
                                    sheet_name: nil, row_offset: 0, cell_offset: 0, key_values: {}, 
                                    verbose: false, create_if_not_exists: false)
    template = Template.process!(template_input, key_values:, verbose:)

    # XXX use create_if_not_exists
    spreadsheet = GoogleSheet.new(sheet_id, sheet_name:, verbose:, row_offset:, cell_offset:)
    spreadsheet.push!(template)
  rescue Google::Apis::ClientError => e
    if verbose
      $stderr.puts "#{e.status_code} Error making Google Sheets API request [#{e.message}]: #{e.body}"
    else
      $stderr.puts "Error making Google Sheets API request: #{e.message}"
    end
    exit 1
  end
end

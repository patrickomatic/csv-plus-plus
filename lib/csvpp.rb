require_relative 'template'
require_relative 'google_sheet'
require_relative 'language/execution_context'

module CSVPlusPlus
  def self.apply_template_to_sheet!(input, filename, google_sheet_id:, sheet_name:, row_offset:, cell_offset:,
                                    key_values:, verbose:, create_if_not_exists:)
    Language::ExecutionContext::with_execution_context(input:, filename:, verbose:)  do |ec|
      template = Template.parse(execution_context: ec, key_values:)

      spreadsheet = GoogleSheet.new(sheet_id, sheet_name:, execution_context: ec,
                                    row_offset:, cell_offset:, create_if_not_exists:)
      spreadsheet.push!(template)
    end
  # TODO move this catch somewhere else? we could have centralized handling in ExecutionContext
  rescue Google::Apis::ClientError => e
    if verbose
      $stderr.puts "#{e.status_code} Error making Google Sheets API request [#{e.message}]: #{e.body}"
    else
      $stderr.puts "Error making Google Sheets API request: #{e.message}"
    end
    exit 1
  end
end

# frozen_string_literal: true

require_relative 'google_sheet'
require_relative 'language/compiler'
require_relative 'template'

# A language for writing rich CSV data
module CSVPlusPlus
  # Create a template and output it using a GoogleSheet
  # rubocop:disable Metrics/MethodLength, Metrics/ParameterLists
  def self.apply_template_to_sheet!(
    input, filename, google_sheet_id:, sheet_name:, row_offset:, cell_offset:,
    key_values:, verbose:, create_if_not_exists:
  )
    ::CSVPlusPlus::Language::Compiler.with_compiler(input:, filename:, verbose:) do |c|
      template = ::CSVPlusPlus::Template.run(compiler: c, key_values:)

      spreadsheet = ::CSVPlusPlus::GoogleSheet.new(
        google_sheet_id,
        sheet_name:,
        row_offset:,
        cell_offset:,
        create_if_not_exists:
      )
      spreadsheet.push!(template)
    end
  # TODO: move this catch somewhere else? we could have centralized handling in ExecutionContext
  rescue ::Google::Apis::ClientError => e
    if verbose
      warn("#{e.status_code} Error making Google Sheets API request [#{e.message}]: #{e.body}")
    else
      warn("Error making Google Sheets API request: #{e.message}")
    end
    exit(1)
  end
  # rubocop:enable Metrics/MethodLength, Metrics/ParameterLists
end

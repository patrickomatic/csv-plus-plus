# frozen_string_literal: true

require_relative 'google_sheet'
require_relative 'language/compiler'
require_relative 'template'

# A language for writing rich CSV data
module CSVPlusPlus
  # Create a template and output it using a GoogleSheet
  def self.apply_template_to_sheet!(input, filename, options)
    ::CSVPlusPlus::Language::Compiler.with_compiler(input:, filename:, options:) do |c|
      template = c.parse_template

      spreadsheet = ::CSVPlusPlus::GoogleSheet.new(options)
      c.outputting! { spreadsheet.push!(template) }
    end
  rescue ::Google::Apis::ClientError => e
    handle_google_error(e, options)
  end

  private

  def handle_google_error(error, options)
    if options.verbose
      warn("#{error.status_code} Error making Google Sheets API request [#{error.message}]: #{error.body}")
    else
      warn("Error making Google Sheets API request: #{error.message}")
    end
    exit(1)
  end
end

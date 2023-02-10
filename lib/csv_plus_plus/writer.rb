# frozen_string_literal: true

require_relative './writer/base_writer'
require_relative './writer/csv'
require_relative './writer/excel'
require_relative './writer/google_sheets'
require_relative './writer/open_document'

module CSVPlusPlus
  # Various strategies for writing to various formats (excel, google sheets, CSV, OpenDocument)
  module Writer
    # Return an instance of a writer depending on the given +options+
    def self.writer(options)
      return ::CSVPlusPlus::Writer::GoogleSheets.new(options) if options.google

      case options.output_filename
      when /\.csv$/ then ::CSVPlusPlus::Writer::CSV.new(options)
      when /\.ods$/ then ::CSVPlusPlus::Writer::OpenDocument.new(options)
      when /\.xlsx?$/ then ::CSVPlusPlus::Writer::Excel.new(options)
      else raise(::StandardError, "Unsupported extension: #{options.output_filename}")
      end
    end
  end
end

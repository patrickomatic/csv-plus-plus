# frozen_string_literal: true

require_relative './base_writer'
require_relative './csv'
require_relative './excel'
require_relative './google_sheets'
require_relative './open_document'

module CSVPlusPlus
  ##
  # Various strategies for writing to various formats (excel, google sheets, CSV, OpenDocument)
  module Writer
    # Return an instance of a writer depending on the given +options+
    def self.writer(options)
      return ::CSVPlusPlus::Writer::GoogleSheets.new(options) if options.google

      case options.output_filename
      when /\.csv$/ then ::CSVPlusPlus::Writer::CSV.new(options)
      when /\.ods$/ then ::CSVPlusPlus::Writer::OpenDocument.new(options)
      when /\.xls$/ then ::CSVPlusPlus::Writer::Excel.new(options)
      else raise(::StandardError, "Unsupported extension: #{options.output_filename}")
      end
    end
  end
end

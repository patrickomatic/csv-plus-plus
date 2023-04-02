# typed: strict
# frozen_string_literal: true

require_relative './writer/base_writer'
require_relative './writer/csv'
require_relative './writer/excel'
require_relative './writer/google_sheets'
require_relative './writer/open_document'

module CSVPlusPlus
  # Various strategies for writing to various formats (excel, google sheets, CSV & OpenDocument (not yet implemented))
  module Writer
    extend ::T::Sig

    sig do
      params(
        options: ::CSVPlusPlus::Options,
        runtime: ::CSVPlusPlus::Runtime::Runtime
      ).returns(
        ::T.any(
          ::CSVPlusPlus::Writer::CSV,
          ::CSVPlusPlus::Writer::Excel,
          ::CSVPlusPlus::Writer::GoogleSheets,
          ::CSVPlusPlus::Writer::OpenDocument
        )
      )
    end
    # Return an instance of a writer depending on the given +options+
    #
    # @param options [Options] The supplied options.
    # @param runtime [Runtime] The current runtime.
    #
    # @return [Writer::CSV | Writer::Excel | Writer::GoogleSheets | Writer::OpenDocument]
    def self.writer(options, runtime)
      return ::CSVPlusPlus::Writer::GoogleSheets.new(options, runtime) if options.google

      case options.output_filename
      when /\.csv$/ then ::CSVPlusPlus::Writer::CSV.new(options, runtime)
      when /\.ods$/ then ::CSVPlusPlus::Writer::OpenDocument.new(options, runtime)
      when /\.xl(sx|sm|tx|tm)$/ then ::CSVPlusPlus::Writer::Excel.new(options, runtime)
      else raise(::CSVPlusPlus::Error::Error, "Unsupported file extension: #{options.output_filename}")
      end
    end
  end
end

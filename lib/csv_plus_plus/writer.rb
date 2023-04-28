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
        position: ::CSVPlusPlus::Runtime::Position
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
    # @param position [Position] The current position.
    #
    # @return [Writer::CSV | Writer::Excel | Writer::GoogleSheets | Writer::OpenDocument]
    # rubocop:disable Metrics/MethodLength
    def self.writer(options, position)
      output_format = options.output_format
      case output_format
      when ::CSVPlusPlus::Options::OutputFormat::CSV then ::CSVPlusPlus::Writer::CSV.new(options, position)
      when ::CSVPlusPlus::Options::OutputFormat::Excel then ::CSVPlusPlus::Writer::Excel.new(options, position)
      when ::CSVPlusPlus::Options::OutputFormat::GoogleSheets then ::CSVPlusPlus::Writer::GoogleSheets.new(
        options,
        position
      )
      when ::CSVPlusPlus::Options::OutputFormat::OpenDocument then ::CSVPlusPlus::Writer::OpenDocument.new(
        options,
        position
      )
      else
        ::T.absurd(output_format)
      end
    end
    # rubocop:enable Metrics/MethodLength
  end
end

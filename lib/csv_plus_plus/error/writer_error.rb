# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # An error that can be thrown when writing a spreadsheet
    class WriterError < ::CSVPlusPlus::Error::Error
      extend ::T::Sig

      sig { override.returns(::String) }
      # @return [::String]
      def error_message
        "Error writing csvpp template: #{message}"
      end
    end
  end
end

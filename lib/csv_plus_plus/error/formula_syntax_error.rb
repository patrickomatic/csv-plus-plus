# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # An error that can be thrown when there is an error parsing a modifier
    #
    # @attr_reader message [::String] A helpful error message
    # @attr_reader bad_input [String] The offending input that caused the error to be thrown
    class FormulaSyntaxError < ::CSVPlusPlus::Error::Error
      extend ::T::Sig
      include ::CSVPlusPlus::Error::PositionalError

      sig { returns(::String) }
      attr_reader :bad_input

      sig { params(message: ::String, bad_input: ::String, wrapped_error: ::T.nilable(::StandardError)).void }
      # @param message [String] A relevant message to show
      # @param bad_input [String] The offending input that caused the error to be thrown
      # @param wrapped_error [StandardError] The underlying error that caused the syntax error.  For example a
      #   Racc::ParseError that was thrown
      def initialize(message, bad_input:, wrapped_error: nil)
        super(message, wrapped_error:)
        @bad_input = bad_input
      end

      sig { override.returns(::String) }
      # Create a relevant error message given +@bad_input+ and +@message+.
      #
      # @return [::String]
      def error_message
        "#{message}: \"#{bad_input}\""
      end
    end
  end
end

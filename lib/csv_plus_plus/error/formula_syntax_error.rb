# frozen_string_literal: true

require_relative './syntax_error'

module CSVPlusPlus
  module Error
    # An error that can be thrown when there is an error parsing a modifier
    #
    # @attr_reader message [::String] A helpful error message
    # @attr_reader bad_input [String] The offending input that caused the error to be thrown
    class FormulaSyntaxError < ::CSVPlusPlus::Error::SyntaxError
      attr_reader :message, :bad_input

      # You must supply either a +choices+ or +message+
      #
      # @param message [String] A relevant message to show
      # @param bad_input [String] The offending input that caused the error to be thrown
      # @param runtime [Runtime] The current runtime
      # @param wrapped_error [StandardError] The underlying error that caused the syntax error.  For example a
      #   Racc::ParseError that was thrown
      def initialize(message, bad_input, runtime, wrapped_error: nil)
        @bad_input = bad_input
        @message = message

        super(runtime, wrapped_error:)
      end

      # Create a relevant error message given +@bad_input+ and +@message+.
      #
      # @return [::String]
      def error_message
        "#{@message}: \"#{@bad_input}\""
      end
    end
  end
end

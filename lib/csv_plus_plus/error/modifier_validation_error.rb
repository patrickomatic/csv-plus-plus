# typed: true
# frozen_string_literal: true

require_relative './syntax_error'

module CSVPlusPlus
  module Error
    # An error that can be thrown when a modifier doesn't pass our validation.
    #
    # @attr_reader modifier [Symbol] The modifier being parsed when the bad input was encountered
    # @attr_reader bad_input [String] The offending input that caused the error to be thrown
    # @attr_reader choices [Array<Symbol>, nil] The choices that +value+ must be one of (but violated)
    # @attr_reader message [String, nil] A relevant message to show
    class ModifierValidationError < ::CSVPlusPlus::Error::Error
      attr_reader :bad_input, :choices, :message, :modifier

      # You must supply either a +choices+ or +message+
      #
      # @param modifier [Symbol] The modifier being parsed when the bad input was encountered
      # @param bad_input [String] The offending input that caused the error to be thrown
      # @param choices [Array<Symbol>, nil] The choices that +value+ must be one of (but violated)
      # @param message [String, nil] A relevant message to show
      def initialize(modifier, bad_input:, choices: nil, message: nil)
        @bad_input = bad_input
        @choices = choices
        @modifier = modifier

        @message =
          if @choices
            "must be one of (#{@choices.map(&:to_s).join(', ')})"
          else
            message
          end

        super(@message)
      end

      # A user-facing error message
      #
      # @return [::String]
      def error_message
        @message
      end
    end
  end
end

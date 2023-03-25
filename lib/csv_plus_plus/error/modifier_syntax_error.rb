# frozen_string_literal: true

require_relative './syntax_error'

module CSVPlusPlus
  module Error
    # An Error that wraps a +ModifierValidationError+ with a +Runtime+.
    class ModifierSyntaxError < ::CSVPlusPlus::Error::SyntaxError
      # @return [ModifierSyntaxError]
      def self.from_validation_error(runtime, modifier_validation_error)
        new(
          runtime,
          modifier_validation_error.modifier,
          bad_input: modifier_validation_error.bad_input,
          message: modifier_validation_error.message,
          wrapped_error: modifier_validation_error
        )
      end

      # You must supply either a +choices+ or +message+
      #
      # @param runtime [Runtime] The current runtime
      # @param wrapped_error [ModifierValidationError] The validtion error that this is wrapping
      def initialize(runtime, bad_input:, message:, modifier: nil, wrapped_error: nil)
        @bad_input = bad_input
        @modifier = modifier
        @message = message

        super(runtime, wrapped_error:)
      end

      # Create a relevant error message given +@choices+ or +@message+ (one of them must be supplied).
      #
      # @return [::String]
      def error_message
        <<~ERROR_MESSAGE
          Error parsing modifier: [[#{@modifier}=...]]
          Bad input: #{@bad_input}
          Reason: #{@message}
        ERROR_MESSAGE
      end
    end
  end
end

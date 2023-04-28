# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # A syntax error encountered when parsing a modifier definition
    class ModifierSyntaxError < ::CSVPlusPlus::Error::Error
      extend ::T::Sig
      include ::CSVPlusPlus::Error::PositionalError

      sig { returns(::String) }
      attr_reader :bad_input

      sig { returns(::T.nilable(::Symbol)) }
      attr_reader :modifier

      sig do
        params(
          message: ::String,
          bad_input: ::String,
          modifier: ::T.nilable(::Symbol),
          wrapped_error: ::T.nilable(::StandardError)
        ).void
      end
      # @param message [String] The error message
      # @param bad_input [String] The offending input
      # @param modifier [Symbol] The modifier being parsed
      # @param wrapped_error [ModifierValidationError] The validtion error that this is wrapping
      def initialize(message, bad_input:, modifier: nil, wrapped_error: nil)
        super(message, wrapped_error:)

        @bad_input = bad_input
        @modifier = modifier
      end

      sig { override.returns(::String) }
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

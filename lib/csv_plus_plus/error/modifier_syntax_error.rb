# typed: strict
# frozen_string_literal: true

require_relative './syntax_error'

module CSVPlusPlus
  module Error
    # An Error that wraps a +ModifierValidationError+ with a +Runtime+.
    class ModifierSyntaxError < ::CSVPlusPlus::Error::SyntaxError
      extend ::T::Sig

      sig { returns(::String) }
      attr_reader :bad_input

      sig { returns(::String) }
      attr_reader :message

      sig { returns(::T.nilable(::Symbol)) }
      attr_reader :modifier

      sig do
        params(
          runtime: ::CSVPlusPlus::Runtime::Runtime,
          modifier_validation_error: ::CSVPlusPlus::Error::ModifierValidationError
        ).returns(::CSVPlusPlus::Error::ModifierSyntaxError)
      end
      # Create a +ModifierSyntaxError+ given a +runtime+ and +ModifierValidationError+.
      #
      # @param runtime [Runtime]
      # @param modifier_validation_error [ModifierValidationError]
      #
      # @return [ModifierSyntaxError]
      def self.from_validation_error(runtime, modifier_validation_error)
        new(
          runtime,
          modifier: modifier_validation_error.modifier,
          bad_input: modifier_validation_error.bad_input,
          message: modifier_validation_error.message,
          wrapped_error: modifier_validation_error
        )
      end

      sig do
        params(
          runtime: ::CSVPlusPlus::Runtime::Runtime,
          bad_input: ::String,
          message: ::String,
          modifier: ::T.nilable(::Symbol),
          wrapped_error: ::T.nilable(::StandardError)
        ).void
      end
      # @param runtime [Runtime] The current runtime
      # @param wrapped_error [ModifierValidationError] The validtion error that this is wrapping
      def initialize(runtime, bad_input:, message:, modifier: nil, wrapped_error: nil)
        @bad_input = bad_input
        @modifier = modifier
        @message = message

        super(runtime, wrapped_error:)
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

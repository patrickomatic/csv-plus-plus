# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # An error that can be thrown when a modifier doesn't pass our validation.
    #
    # @attr_reader modifier [Symbol] The modifier being parsed when the bad input was encountered
    # @attr_reader bad_input [String] The offending input that caused the error to be thrown
    # @attr_reader choices [Array<Symbol>, nil] The choices that +value+ must be one of (but violated)
    # @attr_reader message [String, nil] A relevant message to show
    class ModifierValidationError < ::CSVPlusPlus::Error::ModifierSyntaxError
      extend ::T::Sig
      include ::CSVPlusPlus::Error::PositionalError

      sig do
        params(
          modifier: ::Symbol,
          bad_input: ::String,
          choices: ::T.nilable(::T.class_of(::T::Enum)),
          message: ::T.nilable(::String)
        ).void
      end
      # You must supply either a +choices+ or +message+
      #
      # @param modifier [Symbol] The modifier being parsed when the bad input was encountered
      # @param bad_input [String] The offending input that caused the error to be thrown
      # @param choices [Array<Symbol>, nil] The choices that +value+ must be one of (but violated)
      # @param message [String, nil] A relevant message to show
      def initialize(modifier, bad_input:, choices: nil, message: nil)
        message = ::T.let(
          if choices
            "must be one of (#{choices.values.map(&:serialize).join(', ')})"
          else
            ::T.must(message)
          end,
          ::String
        )
        super(message, bad_input:, modifier:)
      end
    end
  end
end

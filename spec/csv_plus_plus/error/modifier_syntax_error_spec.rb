# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Error::ModifierSyntaxError do
  let(:runtime) { build(:runtime) }

  describe '.from_validation_error' do
    # TODO
  end

  describe '#error_message' do
    let(:bad_input) { 'bad input' }
    let(:choices) { nil }
    let(:message) { 'invalid input' }

    subject { described_class.new(runtime, bad_input:, message:, modifier: :format).error_message }

    it {
      is_expected.to(
        eq(<<~ERROR_MESSAGE))
          Error parsing modifier: [[format=...]]
          Bad input: bad input
          Reason: invalid input
      ERROR_MESSAGE
    }
  end
end

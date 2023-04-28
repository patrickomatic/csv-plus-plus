# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Error::ModifierSyntaxError do
  describe '#error_message' do
    let(:bad_input) { 'bad input' }
    let(:choices) { nil }
    let(:message) { 'invalid input' }

    subject { described_class.new(message, bad_input:, modifier: :format).error_message }

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

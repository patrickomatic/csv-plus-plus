# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Error::ModifierValidationError do
  let(:bad_input) { 'bad input' }
  let(:choices) { nil }
  let(:message) { 'invalid input' }

  subject(:error) { described_class.new(:format, bad_input:, choices:, message:) }

  describe '#initialize' do
    it 'sets @message' do
      expect(subject.message).to(eq('invalid input'))
    end

    context 'with choices' do
      let(:choices) { %w[one two three] }
      let(:message) { nil }

      it 'sets @message based on choices' do
        expect(subject.message).to(eq('must be one of (one, two, three)'))
      end
    end
  end
end

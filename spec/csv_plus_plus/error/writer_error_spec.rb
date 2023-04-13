# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Error::WriterError do
  let(:error) { described_class.new('test') }

  describe '#error_message' do
    subject { error.error_message }

    it { is_expected.to(eq('Error writing csvpp template: test')) }
  end
end

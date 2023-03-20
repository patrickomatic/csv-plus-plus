# frozen_string_literal: true

describe ::CSVPlusPlus::Error::ModifierSyntaxError do
  let(:runtime) { build(:runtime) }
  let(:test_class) do
    ::Class.new do
      def error_message
        'test'
      end
    end
  end

  let(:error) { described_class.new(runtime, wrapped_error: test_class.new) }

  describe '#error_message' do
    subject { error.error_message }

    it { is_expected.to(eq('test')) }
  end
end

# frozen_string_literal: true

describe ::CSVPlusPlus::Expand do
  let(:repetitions) { 2 }

  subject(:expand) { described_class.new(repetitions) }

  describe '#infinite?' do
    it { is_expected.not_to(be_infinite) }

    context 'with repetitions = nil' do
      let(:repetitions) { nil }

      it { is_expected.to(be_infinite) }
    end
  end

  describe '#to_s' do
    subject { expand.to_s }

    it { is_expected.to(eq('Expand 2')) }

    context 'when infinite' do
      let(:repetitions) { nil }

      it { is_expected.to(eq('Expand infinity')) }
    end
  end
end

# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Expand do
  let(:repetitions) { 2 }

  subject(:expand) { build(:expand, repetitions:) }

  describe '#expanded?' do
    it { is_expected.not_to(be_expanded) }

    context 'with starts_at set' do
      before { expand.starts_at = 5 }

      it { is_expected.to(be_expanded) }
    end
  end

  describe '#infinite?' do
    it { is_expected.not_to(be_infinite) }

    context 'with repetitions = nil' do
      let(:repetitions) { nil }

      it { is_expected.to(be_infinite) }
    end
  end

  describe '#starts_at=' do
    before { expand.starts_at = 5 }

    it 'sets starts_at and ends_at' do
      expect(subject.starts_at).to(eq(5))
      expect(subject.ends_at).to(eq(7))
    end

    context 'with repetitions = nil' do
      let(:repetitions) { nil }

      it 'sets starts_at but not ends_at' do
        expect(subject.starts_at).to(eq(5))
        expect(subject.ends_at).to(be_nil)
      end
    end
  end
end

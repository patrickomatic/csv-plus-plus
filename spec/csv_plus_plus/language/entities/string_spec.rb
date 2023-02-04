# frozen_string_literal: true

describe ::CSVPlusPlus::Language::Entities::String do
  subject(:string) { described_class.new('foo') }

  describe '#initialize' do
    it 'has a nil id' do
      expect(subject.id).to(be_nil)
    end
  end

  describe '#string?' do
    it { is_expected.to(be_string) }
  end

  describe '#to_s' do
    subject { string.to_s }

    it { is_expected.to(eq('"foo"')) }
  end

  describe '#==' do
    it { is_expected.to(eq(build(:string, s: 'foo'))) }

    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end
end

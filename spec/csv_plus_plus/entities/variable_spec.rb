# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::Variable do
  subject(:entity) { described_class.new('RowNUM') }

  describe '#initialize' do
    it 'lowercases and converts the id to a symbol' do
      expect(subject.id).to(eq(:rownum))
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:variable, id: :rownum))) }

    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#evaluate' do
    let(:runtime) { build(:runtime) }

    subject { entity.evaluate(runtime) }

    it { is_expected.to(eq('$$rownum')) }
  end

  describe '#variable?' do
    it { is_expected.to(be_variable) }
  end
end

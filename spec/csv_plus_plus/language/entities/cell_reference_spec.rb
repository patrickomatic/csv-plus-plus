# frozen_string_literal: true

describe ::CSVPlusPlus::Language::Entities::CellReference do
  subject(:cell_reference) { described_class.new('A1') }

  describe '#initialize' do
    it 'lowercases and converts the id to a symbol' do
      expect(subject.cell_reference).to(eq('A1'))
    end
  end

  describe '#to_s' do
    subject { cell_reference.to_s }

    it { is_expected.to(eq('A1')) }
  end

  describe '#cell_reference?' do
    it { is_expected.to(be_cell_reference) }
  end

  describe '#variable?' do
    it { is_expected.not_to(be_variable) }
  end

  describe '#==' do
    it { is_expected.to(eq(build(:cell_reference, ref: 'A1'))) }

    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end
end

# frozen_string_literal: true

require 'entities'

describe ::CSVPlusPlus::Language::CellReference do
  subject { described_class.new('A1') }

  describe '#initialize' do
    it 'lowercases and converts the id to a symbol' do
      expect(subject.id).to(eq(:a1))
    end
  end

  describe '#cell_reference?' do
    it { is_expected.to(be_cell_reference) }
  end

  describe '#==' do
    it { is_expected.to(eq(build(:cell_reference, ref: :a1))) }

    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end
end

# frozen_string_literal: true

require 'row'

describe ::CSVPlusPlus::Row do
  describe '#expand_amount' do
    let(:index) { 0 }
    let(:expand_amount) { 2 }
    let(:modifier) { build(:modifier_with_expand, repetitions: expand_amount) }
    let(:row) { build(:row, modifier:, index:) }

    subject { row.expand_amount }

    it { is_expected.to(eq(2)) }

    context 'when no amount is set' do
      let(:expand_amount) { nil }

      it { is_expected.to(eq(1000)) }

      context 'and the row is offset' do
        let(:index) { 2 }
        it { is_expected.to(eq(998)) }
      end
    end
  end

  describe '#index=' do
    let(:expand_amount) { 2 }
    let(:row_index) { 0 }
    let(:cells) do
      [
        build(:cell, row_index:, index: 0, value: 'foo'),
        build(:cell, row_index:, index: 1, value: 'bar'),
        build(:cell, row_index:, index: 2, value: 'baz')
      ]
    end
    let(:modifier) { build(:modifier) }
    let(:row) { described_class.new(row_index, cells, modifier) }

    before { row.index = 10 }

    it 'sets the value' do
      expect(row.index).to(eq(10))
    end

    it 'propagates the change to each cell.row_index' do
      expect(row.cells[0].row_index).to(eq(10))
      expect(row.cells[1].row_index).to(eq(10))
      expect(row.cells[2].row_index).to(eq(10))
    end
  end

  describe '#to_s' do
    let(:modifier) { build(:modifier_with_expand, repetitions: 2) }
    let(:row) { build(:row, modifier:, index: 0, cells:) }
    let(:cells) do
      [
        build(:cell, row_index: 0, index: 0, value: 'foo'),
        build(:cell, row_index: 0, index: 1, value: 'bar'),
        build(:cell, row_index: 0, index: 2, value: 'baz')
      ]
    end

    subject { row.to_s }

    it { is_expected.to(match(/Row\(index: 0, modifier: Modifier\(.+\), cells: .*\)/)) }
  end
end

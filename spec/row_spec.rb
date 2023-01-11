# frozen_string_literal: true

require 'row'

describe ::CSVPlusPlus::Row do
  let(:execution_context) { build(:execution_context) }

  describe '#parse' do
    let(:values) { %w[foo bar baz] }

    subject(:row) { described_class.parse(values, execution_context) }

    it { is_expected.to(be_a(described_class)) }

    it 'sets rows.index' do
      expect(row.index).to(eq(0))
    end

    it 'sets cell.index' do
      expect(row.cells[0].index).to(eq(0))
      expect(row.cells[1].index).to(eq(1))
      expect(row.cells[2].index).to(eq(2))
    end

    it 'sets cell.row_index' do
      expect(row.cells[0].row_index).to(eq(row.index))
      expect(row.cells[1].row_index).to(eq(row.index))
      expect(row.cells[2].row_index).to(eq(row.index))
    end

    context 'with a cell modifier' do
      let(:values) { ['[[format=bold]]foo', 'bar', 'baz'] }

      it 'does not set the modifier on the row' do
        expect(row.modifier).not_to(be_formatted('bold'))
      end

      it 'sets bold only on one cell' do
        expect(row.cells[0].modifier).to(be_formatted('bold'))
        expect(row.cells[1].modifier).not_to(be_formatted('bold'))
        expect(row.cells[2].modifier).not_to(be_formatted('bold'))
      end
    end

    describe 'a row modifier provides defaults for the row' do
      let(:values) { ['![[format=bold]]foo', 'bar', 'baz'] }

      it 'sets bold on the row' do
        expect(row.modifier).to(be_formatted('bold'))
      end

      it 'sets bold on each cell' do
        expect(row.cells[0].modifier).to(be_formatted('bold'))
        expect(row.cells[1].modifier).to(be_formatted('bold'))
        expect(row.cells[2].modifier).to(be_formatted('bold'))
      end
    end
  end

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
end

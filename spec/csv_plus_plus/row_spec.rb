# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Row do
  describe '#expand_amount' do
    let(:index) { 0 }
    let(:modifier) { build(:modifier_with_expand) }
    let(:row) { build(:row, modifier:, index:) }

    subject { row.expand_amount }

    it { is_expected.to(eq(2)) }

    context 'when no amount is set' do
      let(:modifier) { build(:modifier_with_infinite_expand) }

      it { is_expected.to(eq(1000)) }

      context 'and the row is offset' do
        let(:index) { 2 }

        it { is_expected.to(eq(998)) }
      end
    end
  end

  describe '#expand_rows' do
    let(:modifier) { build(:modifier_with_expand) }
    let(:row) { build(:row, modifier:, index: 5) }
    let(:starts_at) { 0 }
    let(:into) { [] }

    subject { row.expand_rows(starts_at:, into:) }

    it 'fills :into with the expanded rows' do
      subject
      expect(into.length).to(eq(2))
    end

    it 'also returns :into' do
      expect(subject).to(be(into))
    end
  end

  describe '#index=' do
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

  describe '#unexpanded?' do
    let(:modifier) { build(:modifier) }
    let(:row) { build(:row, modifier:) }

    subject { row.unexpanded? }

    context 'without an expand modifier' do
      it { is_expected.to(be(false)) }
    end

    context 'with an expand modifier that has not been expanded' do
      let(:modifier) { build(:modifier_with_expand) }

      it { is_expected.to(be(true)) }
    end

    context 'with an expand modifier that has been expanded' do
      let(:modifier) { build(:modifier_with_expand) }

      before { modifier.expand.starts_at = 5 }

      it { is_expected.to(be(false)) }
    end
  end
end

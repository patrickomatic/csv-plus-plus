# frozen_string_literal: true

describe ::CSVPlusPlus::ModifierParser do
  let(:runtime) { build(:runtime) }
  let(:scope) { build(:scope) }

  describe '#parse' do
    let(:row_modifier) { build(:row_modifier) }
    let(:cell_modifier) { build(:modifier) }
    let(:rest) { described_class.new(cell_modifier:, row_modifier:, scope:).parse(value, runtime) }

    before(:each) { rest }

    context 'without a modifier' do
      let(:value) { 'foo' }

      subject { rest }

      it { is_expected.to(eq('foo')) }
    end

    context 'multiple modifiers' do
      let(:value) { '[[halign=left/format=bold/format=underline]]=A + B' }

      subject { rest }

      it { is_expected.to(eq('=A + B')) }

      it 'updates cell_modifier' do
        expect(cell_modifier).to(be_formatted('bold'))
        expect(cell_modifier).to(be_formatted('underline'))
        expect(cell_modifier.halign).to(eq('left'))
      end
    end

    context 'row-based modifiers' do
      let(:value) { '![[valign=center / format=bold]]Stocks' }

      subject { rest }

      it { is_expected.to(eq('Stocks')) }

      it 'updates row_modifier' do
        expect(row_modifier).to(be_formatted('bold'))
        expect(row_modifier.valign).to(eq('center'))
      end
    end

    context 'a row and a cell modifier' do
      let(:value) { '![[valign=center/format=bold]][[format=underline]]Stocks' }

      subject { rest }

      it { is_expected.to(eq('Stocks')) }

      it 'parses the row modifier' do
        expect(row_modifier).to(be_formatted('bold'))
        expect(row_modifier.valign).to(eq('center'))
      end

      it 'also parses the cell modifier and applies the row modifier' do
        expect(cell_modifier).to(be_formatted('bold'))
        expect(cell_modifier).to(be_formatted('underline'))
        expect(cell_modifier.valign).to(eq('center'))
      end
    end

    describe 'border' do
      let(:value) { '[[border=top/border=bottom]]=ADD(1, 2)' }

      subject { cell_modifier.borders }

      it { is_expected.to(include('top')) }
      it { is_expected.to(include('bottom')) }
    end

    describe 'color' do
      let(:value) { '[[color=#FF00FF]]=ADD(1, 2)' }

      subject { cell_modifier.color }

      it { is_expected.to(eq('#FF00FF')) }
    end

    describe 'expand' do
      let(:value) { '![[expand=5]]foo' }

      subject { row_modifier.expand }

      it { is_expected.to(eq('5')) }

      context 'with an infinite expand' do
        let(:value) { '![[expand]]foo' }

        it { is_expected.to(be_infinite) }
      end

      # TODO: we should have a check somewhere so that you can't have a expand= on a cell modifier (only on a row)
    end

    describe 'halign' do
      let(:value) { '[[halign=left]]foo' }

      subject { cell_modifier.halign }

      it { is_expected.to(eq('left')) }
    end

    describe 'note' do
      let(:value) { "[[note='this is a note']]=A + B" }

      subject { cell_modifier.note }

      it { is_expected.to(eq('this is a note')) }
    end

    describe 'var' do
      let(:value) { '[[var=foo]]foo' }

      subject { cell_modifier.var }

      it { is_expected.to(eq(:foo)) }

      it 'defines a variable' do
        expect(scope).to(be_defined_variable(:foo))
      end
    end
  end
end

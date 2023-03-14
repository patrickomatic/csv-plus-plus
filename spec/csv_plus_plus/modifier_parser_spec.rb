# frozen_string_literal: true

describe ::CSVPlusPlus::ModifierParser do
  let(:runtime) { build(:runtime) }
  let(:scope) { build(:scope) }

  describe '#parse' do
    let(:row_modifier) { build(:modifier) }
    let(:cell_modifier) { build(:modifier) }

    before(:each) { subject }
    subject { described_class.new(cell_modifier:, row_modifier:, scope:).parse(value, runtime) }

    context 'without a modifier' do
      let(:value) { 'foo' }

      it { is_expected.to(eq('foo')) }
    end

    context 'one modifier' do
      let(:value) { '[[halign=left]]foo' }

      it { is_expected.to(eq('foo')) }

      it 'updates the cell_modifier with halign=left' do
        expect(cell_modifier.halign).to(eq('left'))
      end
    end

    context 'multiple modifiers' do
      let(:value) { '[[halign=left/format=bold/format=underline]]=A + B' }

      it { is_expected.to(eq('=A + B')) }

      it 'updates cell_modifier' do
        expect(cell_modifier).to(be_formatted('bold'))
        expect(cell_modifier).to(be_formatted('underline'))
        expect(cell_modifier.halign).to(eq('left'))
      end
    end

    context 'row-based modifiers' do
      let(:value) { '![[valign=center / format=bold]]Stocks' }

      it { is_expected.to(eq('Stocks')) }

      it 'updates row_modifier' do
        expect(row_modifier).to(be_formatted('bold'))
        expect(row_modifier.valign).to(eq('center'))
      end
    end

    context 'a note' do
      let(:value) { "[[note='this is a note']]=A + B" }

      it { is_expected.to(eq('=A + B')) }

      it 'sets the note' do
        expect(cell_modifier.note).to(eq('this is a note'))
      end
    end

    context 'a color' do
      let(:value) { '[[color=#FF00FF]]=ADD(1, 2)' }

      it { is_expected.to(eq('=ADD(1, 2)')) }

      it 'sets the color' do
        expect(cell_modifier.color.to_s).to(eq('Color(r: FF, g: 00, b: FF)'))
      end
    end

    context 'a row and a cell modifier' do
      let(:value) { '![[valign=center/format=bold]][[format=underline]]Stocks' }

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
  end
end

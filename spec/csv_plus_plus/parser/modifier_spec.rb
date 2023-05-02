# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Parser::Modifier do
  describe '#parse' do
    let(:row_modifier) { build(:row_modifier) }
    let(:cell_modifier) { build(:modifier) }
    let(:rest) { described_class.new(cell_modifier:, row_modifier:).parse(value) }

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
        expect(cell_modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
        expect(cell_modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Underline))
        expect(cell_modifier.halign).to(eq(::CSVPlusPlus::Modifier::HorizontalAlign::Left))
      end
    end

    context 'row-based modifiers' do
      let(:value) { '![[valign=center / format=bold]]Stocks' }

      subject { rest }

      it { is_expected.to(eq('Stocks')) }

      it 'updates row_modifier' do
        expect(row_modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
        expect(row_modifier.valign).to(eq(::CSVPlusPlus::Modifier::VerticalAlign::Center))
      end
    end

    context 'a row and a cell modifier' do
      let(:value) { '![[valign=center/format=bold]][[format=underline]]Stocks' }

      subject { rest }

      it { is_expected.to(eq('Stocks')) }

      it 'parses the row modifier' do
        expect(row_modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
        expect(row_modifier.valign).to(eq(::CSVPlusPlus::Modifier::VerticalAlign::Center))
      end

      it 'also parses the cell modifier and applies the row modifier' do
        expect(cell_modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
        expect(cell_modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Underline))
        expect(cell_modifier.valign).to(eq(::CSVPlusPlus::Modifier::VerticalAlign::Center))
      end
    end

    describe 'border' do
      let(:value) { '[[border=top/border=bottom]]=ADD(1, 2)' }

      subject { cell_modifier.borders }

      it { is_expected.to(include(::CSVPlusPlus::Modifier::BorderSide::Top)) }
      it { is_expected.to(include(::CSVPlusPlus::Modifier::BorderSide::Bottom)) }
    end

    describe 'color' do
      let(:value) { '[[color=FF00FF]]=ADD(1, 2)' }

      subject { cell_modifier.color }

      it 'creates a color object' do
        expect(subject.red_hex).to(eq('FF'))
        expect(subject.blue_hex).to(eq('FF'))
        expect(subject.green_hex).to(eq('00'))
      end
    end

    describe 'expand' do
      let(:value) { '![[expand=5]]foo' }

      subject { row_modifier.expand }

      it 'creates an Expand and sets repetitions' do
        expect(subject.repetitions).to(eq(5))
      end

      context 'with an infinite expand' do
        let(:value) { '![[expand]]foo' }

        it { is_expected.to(be_infinite) }
      end

      # TODO: we should have a check somewhere so that you can't have a expand= on a cell modifier (only on a row)
    end

    describe 'format' do
      let(:value) { '[[format=bold]]foo' }

      subject { cell_modifier }

      it { is_expected.to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold)) }
    end

    describe 'halign' do
      let(:value) { '[[halign=left]]foo' }

      subject { cell_modifier.halign }

      it { is_expected.to(eq(::CSVPlusPlus::Modifier::HorizontalAlign::Left)) }
    end

    describe 'note' do
      let(:value) { "[[note='this is a note']]=A + B" }

      subject { cell_modifier.note }

      it { is_expected.to(eq("'this is a note'")) }
    end

    describe 'validate' do
      subject { cell_modifier.validate }

      context 'with a condition that takes no args' do
        let(:value) { '[[validate=blank]]=A + B' }

        it 'parses the validation' do
          expect(subject.condition).to(eq(:blank))
        end
      end

      context 'with a condition that takes an argument' do
        let(:value) { '[[validate=number_eq:1]]=A + B' }

        it 'parses the validation' do
          expect(subject.condition).to(eq(:number_eq))
          expect(subject.arguments).to(eq(['1']))
        end
      end

      context 'with a condition that takes multiple arguments' do
        let(:value) { '[[validate=number_between:10 20]]=A + B' }

        it 'parses the validation' do
          expect(subject.condition).to(eq(:number_between))
          expect(subject.arguments).to(eq(%w[10 20]))
        end
      end

      context 'with a condition that needs to be quoted' do
        let(:value) { "[[validate=date_between:'1/10/22 2/10/22']]=A + B" }

        it 'parses the validation', focus: true do
          expect(subject.condition).to(eq(:date_between))
          expect(subject.arguments).to(eq(['1/10/22', '2/10/22']))
        end
      end
    end

    describe 'var' do
      let(:value) { '[[var=foo]]foo' }

      subject { cell_modifier.var }

      it { is_expected.to(eq(:foo)) }
    end
  end
end

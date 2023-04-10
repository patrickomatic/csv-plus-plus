# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Modifier::Modifier do
  subject(:modifier) { build(:modifier) }

  describe '#any_border?' do
    it { is_expected.not_to(be_any_border) }

    context 'with a border set' do
      before { subject.border = ::CSVPlusPlus::Modifier::BorderSide::Right }

      it { is_expected.to(be_any_border) }
    end
  end

  describe '#border=' do
    context 'with a single values' do
      before do
        modifier.border = ::CSVPlusPlus::Modifier::BorderSide::Top
        modifier.border = ::CSVPlusPlus::Modifier::BorderSide::Left
      end

      it 'sets top & left borders' do
        expect(modifier).to(be_border_along(::CSVPlusPlus::Modifier::BorderSide::Top))
        expect(modifier).to(be_border_along(::CSVPlusPlus::Modifier::BorderSide::Left))
      end
    end

    context 'with all' do
      before { subject.border = ::CSVPlusPlus::Modifier::BorderSide::All }

      it 'sets all borders' do
        expect(modifier).to(be_border_along(::CSVPlusPlus::Modifier::BorderSide::Top))
        expect(modifier).to(be_border_along(::CSVPlusPlus::Modifier::BorderSide::Left))
        expect(modifier).to(be_border_along(::CSVPlusPlus::Modifier::BorderSide::Right))
        expect(modifier).to(be_border_along(::CSVPlusPlus::Modifier::BorderSide::Bottom))
      end
    end
  end

  describe '#border_all?' do
    subject { modifier }

    it { is_expected.not_to(be_border_all) }

    context 'with one border set' do
      before { subject.border = ::CSVPlusPlus::Modifier::BorderSide::Top }

      it { is_expected.not_to(be_border_all) }
    end

    context 'with border = :all' do
      before { subject.border = ::CSVPlusPlus::Modifier::BorderSide::All }

      it { is_expected.to(be_border_all) }
    end

    context 'with all of the borders set individually' do
      before do
        modifier.border = ::CSVPlusPlus::Modifier::BorderSide::Top
        modifier.border = ::CSVPlusPlus::Modifier::BorderSide::Bottom
        modifier.border = ::CSVPlusPlus::Modifier::BorderSide::Left
        modifier.border = ::CSVPlusPlus::Modifier::BorderSide::Right
      end

      it { is_expected.to(be_border_all) }
    end
  end

  describe '#borderstyle' do
    subject { modifier.borderstyle }

    it { is_expected.to(eq(::CSVPlusPlus::Modifier::BorderStyle::Solid)) }

    context 'when set to dashed' do
      before { modifier.borderstyle = ::CSVPlusPlus::Modifier::BorderStyle::Dashed }

      it { is_expected.to(eq(::CSVPlusPlus::Modifier::BorderStyle::Dashed)) }
    end
  end

  describe '#cell_level?' do
    context 'with a cell modifier' do
      it { is_expected.to(be_cell_level) }
    end

    context 'with a row modifier' do
      subject { build(:row_modifier) }

      it { is_expected.not_to(be_cell_level) }
    end
  end

  describe '#infinite_expand!' do
    let(:modifier) { build(:modifier, row_level: true) }

    subject { modifier.expand }

    before { modifier.infinite_expand! }

    it { is_expected.to(be_infinite) }
  end

  describe '#expand=' do
    let(:modifier) { build(:modifier, expand: build(:expand, repetitions: 2)) }

    subject { modifier.expand }

    it { is_expected.not_to(be_infinite) }
  end

  describe '#format=' do
    context 'with a single values' do
      before do
        modifier.format = ::CSVPlusPlus::Modifier::TextFormat::Bold
        modifier.format = ::CSVPlusPlus::Modifier::TextFormat::Strikethrough
      end

      it 'sets formats' do
        expect(modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
        expect(modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Strikethrough))
      end
    end
  end

  describe '#freeze!' do
    context 'by default' do
      it { is_expected.not_to(be_frozen) }
    end

    context 'after calling #freeze!' do
      before { modifier.freeze! }

      it { is_expected.to(be_frozen) }
    end
  end

  describe '#row_level?' do
    context 'with a cell modifier' do
      it { is_expected.not_to(be_row_level) }
    end

    context 'with a row modifier' do
      subject { build(:row_modifier) }

      it { is_expected.to(be_row_level) }
    end
  end

  describe '#row_level!' do
    context 'makes it row_level?' do
      before { subject.row_level! }

      it { is_expected.to(be_row_level) }
    end
  end

  describe '#take_defaults_from!' do
    let(:modifier) { build(:row_modifier) }
    let(:other_modifier) { build(:modifier) }

    before do
      modifier.format = ::CSVPlusPlus::Modifier::TextFormat::Bold
      modifier.format = ::CSVPlusPlus::Modifier::TextFormat::Underline
      modifier.border = ::CSVPlusPlus::Modifier::BorderSide::Top
      modifier.borderstyle = ::CSVPlusPlus::Modifier::BorderStyle::Dotted
      modifier.fontcolor = ::CSVPlusPlus::Color.new('#00FF00')

      other_modifier.take_defaults_from!(modifier)
    end

    it 'copies values from modifier onto other_modifier' do
      expect(other_modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
      expect(other_modifier).to(be_formatted(::CSVPlusPlus::Modifier::TextFormat::Bold))
      expect(other_modifier).to(be_border_along(::CSVPlusPlus::Modifier::BorderSide::Top))
      expect(other_modifier.borderstyle).to(eq(::CSVPlusPlus::Modifier::BorderStyle::Dotted))
      expect(other_modifier.fontcolor).to(eq(modifier.fontcolor))
    end

    it 'does not take row-specific values' do
      expect(other_modifier).not_to(be_row_level)
    end
  end
end

# frozen_string_literal: true

describe ::CSVPlusPlus::Modifier do
  subject(:modifier) { build(:modifier) }

  describe '#any_border?' do
    it { is_expected.not_to(be_any_border) }

    context 'with a border set' do
      before { subject.border = 'right' }

      it { is_expected.to(be_any_border) }
    end
  end

  describe '#border=' do
    context 'with a single values' do
      before do
        modifier.border = 'top'
        modifier.border = 'left'
      end

      it 'sets top & left borders' do
        expect(modifier).to(be_border_along('top'))
        expect(modifier).to(be_border_along('left'))
      end
    end

    context "with 'all'" do
      before { modifier.border = 'all' }

      it 'sets all borders' do
        expect(modifier).to(be_border_along('top'))
        expect(modifier).to(be_border_along('left'))
        expect(modifier).to(be_border_along('right'))
        expect(modifier).to(be_border_along('bottom'))
      end
    end
  end

  describe '#border_all?' do
    subject { modifier }

    it { is_expected.not_to(be_border_all) }

    context 'with one border set' do
      before { modifier.border = 'top' }
      it { is_expected.not_to(be_border_all) }
    end

    context 'with border = "all"' do
      before { modifier.border = 'all' }
      it { is_expected.to(be_border_all) }
    end

    context 'with all of the borders set individually' do
      before do
        modifier.border = 'top'
        modifier.border = 'bottom'
        modifier.border = 'left'
        modifier.border = 'right'
      end
      it { is_expected.to(be_border_all) }
    end
  end

  describe '#bordercolor=' do
    before { modifier.bordercolor = '#FF0000' }

    it 'sets the border color' do
      expect(modifier.bordercolor).to(be_a(::CSVPlusPlus::Color))
    end
  end

  describe '#borderstyle' do
    subject { modifier.borderstyle }

    it { is_expected.to(eq('solid')) }

    context 'when set to dashed' do
      before { modifier.borderstyle = 'dashed' }
      it { is_expected.to(eq('dashed')) }
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

  describe '#color=' do
    before { modifier.color = hex_value }
    let(:hex_value) { '#FF00FF' }

    it 'sets the color attribute' do
      expect(modifier.color).to(be_a(::CSVPlusPlus::Color))
    end
  end

  describe '#expand=' do
    let(:amount) { nil }
    before { modifier.expand = expand }
    subject(:expand) { ::CSVPlusPlus::Expand.new(amount) }

    it { is_expected.to(be_infinite) }

    context 'with an amount' do
      let(:amount) { 2 }

      it { is_expected.not_to(be_infinite) }
    end
  end

  describe '#fontcolor=' do
    before { modifier.fontcolor = '#FF0000' }

    it 'sets the font color' do
      expect(modifier.fontcolor).to(be_a(::CSVPlusPlus::Color))
    end
  end

  describe '#format=' do
    context 'with a single values' do
      before do
        modifier.format = 'bold'
        modifier.format = 'strikethrough'
      end

      it 'sets formats' do
        expect(modifier).to(be_formatted('bold'))
        expect(modifier).to(be_formatted('strikethrough'))
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
      modifier.format = 'bold'
      modifier.format = 'underline'
      modifier.border = 'top'
      modifier.borderstyle = 'thin'
      modifier.fontcolor = '#00FF00'

      other_modifier.take_defaults_from!(modifier)
    end

    it 'copies values from modifier onto other_modifier' do
      expect(other_modifier).to(be_formatted('bold'))
      expect(other_modifier).to(be_formatted('underline'))
      expect(other_modifier).to(be_border_along('top'))
      expect(other_modifier.borderstyle).to(eq('thin'))
      expect(other_modifier.fontcolor).to(eq(modifier.fontcolor))
    end

    it 'does not take row-specific values' do
      expect(other_modifier).not_to(be_row_level)
    end
  end
end

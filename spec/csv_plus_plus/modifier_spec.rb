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

    it 'sets the red, green, blue values' do
      expect(modifier.color.red).to(eq(1))
      expect(modifier.color.green).to(eq(0))
      expect(modifier.color.blue).to(eq(1))
    end

    context 'with a 3-digit hex value' do
      let(:hex_value) { '#F0F' }

      it 'sets the red, green, blue values' do
        expect(modifier.color.red).to(eq(1))
        expect(modifier.color.green).to(eq(0))
        expect(modifier.color.blue).to(eq(1))
      end
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
end

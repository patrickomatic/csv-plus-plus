# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Modifier::GoogleSheetModifier do
  let(:modifier) { described_class.new }

  describe '#background_color' do
    subject { modifier.background_color }

    it { is_expected.to(be_nil) }

    context 'when @color is set' do
      before { modifier.color = ::CSVPlusPlus::Color.new('#FF00AA') }

      it 'sets the RGB color components' do
        expect(subject.alpha).to(be_nil)
        expect(subject.red).to(eq(1))
        expect(subject.blue).to(eq(2.0 / 3.0))
        expect(subject.green).to(eq(0))
      end
    end
  end

  describe '#border' do
    subject { modifier.border }

    it { is_expected.to(be_nil) }

    context 'when @border is set' do
      before do
        modifier.border = ::CSVPlusPlus::Modifier::BorderSide::Top
        modifier.bordercolor = ::CSVPlusPlus::Color.new('#00FF00')
        modifier.borderstyle = ::CSVPlusPlus::Modifier::BorderStyle::SolidThick
      end

      it 'sets the border color components' do
        expect(subject.color.blue).to(eq(0))
        expect(subject.color.green).to(eq(1))
        expect(subject.color.red).to(eq(0))
      end

      it 'sets the border style' do
        expect(subject.style).to(eq('SOLID_THICK'))
      end
    end
  end

  describe '#font_color' do
    subject { modifier.font_color }

    it { is_expected.to(be_nil) }

    context 'when @fontcolor is set' do
      before { modifier.fontcolor = ::CSVPlusPlus::Color.new('#FF00AA') }

      it 'sets the RGB color components' do
        expect(subject.alpha).to(be_nil)
        expect(subject.red).to(eq(1))
        expect(subject.blue).to(eq(2.0 / 3.0))
        expect(subject.green).to(eq(0))
      end
    end
  end

  describe '#horizontal_alignment' do
    subject { modifier.horizontal_alignment }

    it { is_expected.to(be_nil) }

    context 'when @halign is set' do
      before { modifier.halign = ::CSVPlusPlus::Modifier::HorizontalAlign::Left }

      it { is_expected.to(eq('LEFT')) }
    end
  end

  describe '#number_format' do
    subject { modifier.number_format }

    it { is_expected.to(be_nil) }

    context 'when @numberformat is set' do
      before { modifier.numberformat = ::CSVPlusPlus::Modifier::NumberFormat::DateTime }

      it 'sets type on NumberFormat' do
        expect(subject.type).to(eq('DATE_TIME'))
      end
    end
  end

  describe '#text_format' do
    subject { modifier.text_format }

    before do
      modifier.format = ::CSVPlusPlus::Modifier::TextFormat::Bold
      modifier.format = ::CSVPlusPlus::Modifier::TextFormat::Underline
      modifier.fontfamily = 'Foovetica'
      modifier.fontsize = 20
      modifier.fontcolor = ::CSVPlusPlus::Color.new('#AABBCC')
    end

    it { is_expected.not_to(be_nil) }

    it 'sets the properties of the TextFormat' do
      expect(subject.bold).to(eq(true))
      expect(subject.underline).to(eq(true))
      expect(subject.font_family).to(eq('Foovetica'))
      expect(subject.font_size).to(eq(20))
      expect(subject.foreground_color).to(be_a(::Google::Apis::SheetsV4::Color))
    end
  end

  describe '#vertical_alignment' do
    subject { modifier.vertical_alignment }

    it { is_expected.to(be_nil) }

    context 'when @valign is set' do
      before { modifier.valign = ::CSVPlusPlus::Modifier::VerticalAlign::Bottom }

      it { is_expected.to(eq('BOTTOM')) }
    end
  end
end

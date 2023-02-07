# frozen_string_literal: true

describe ::CSVPlusPlus::Color do
  let(:hex_string) { '#FF0001' }
  let(:color) { ::CSVPlusPlus::Color.new(hex_string) }

  describe '#initialize' do
    it 'sets @red component' do
      expect(color.red).to(eq(1))
    end

    it 'sets @green component' do
      expect(color.green).to(eq(0))
    end

    it 'sets @blue component' do
      expect(color.blue).to(eq(1 / 255.0))
    end

    context 'with an invalid hex_string' do
      let(:hex_string) { 'invalid' }

      it 'sets @red component to 0' do
        expect(color.red).to(eq(0))
      end

      it 'sets @green component to 0' do
        expect(color.green).to(eq(0))
      end

      it 'sets @blue component to 0' do
        expect(color.blue).to(eq(0))
      end
    end
  end

  describe '#to_s' do
    subject { color.to_s }

    it { is_expected.to(eq('Color(r: 1.0, g: 0.0, b: 0.00392156862745098)')) }
  end

  describe '#==' do
    it 'is true for two instances that are the same' do
      expect(color == described_class.new(hex_string)).to(be(true))
    end

    it 'is false for two instances that are different' do
      expect(color == described_class.new('#111111')).to(be(false))
    end

    it 'is false for non-instances' do
      expect(color == 'a string').to(be(false))
    end
  end
end

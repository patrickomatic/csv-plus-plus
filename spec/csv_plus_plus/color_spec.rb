# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Color do
  let(:hex_string) { '#FF0001' }
  let(:color) { ::CSVPlusPlus::Color.new(hex_string) }

  describe '#initialize' do
    it 'sets @red_hex component' do
      expect(color.red_hex).to(eq('FF'))
    end

    it 'sets @green_hex component' do
      expect(color.green_hex).to(eq('00'))
    end

    it 'sets @blue_hex component' do
      expect(color.blue_hex).to(eq('01'))
    end

    context 'with a 3-character hex string' do
      let(:hex_string) { 'FA2' }

      it 'sets @red_hex component' do
        expect(color.red_hex).to(eq('FF'))
      end

      it 'sets @green_hex component' do
        expect(color.green_hex).to(eq('AA'))
      end

      it 'sets @blue_hex component' do
        expect(color.blue_hex).to(eq('22'))
      end
    end

    context 'with an invalid hex string' do
      let(:hex_string) { 'invalid' }

      it 'sets @red_hex component to 0' do
        expect(color.red_hex).to(be_nil)
      end

      it 'sets @green_hex component to 0' do
        expect(color.green_hex).to(be_nil)
      end

      it 'sets @blue_hex component to 0' do
        expect(color.blue_hex).to(be_nil)
      end
    end
  end

  describe '#blue_percent' do
    subject { color.blue_percent }
    it { is_expected.to(eq(0.00392156862745098)) }
  end

  describe '#green_percent' do
    subject { color.green_percent }
    it { is_expected.to(eq(0.0)) }
  end

  describe '#red_percent' do
    subject { color.red_percent }
    it { is_expected.to(eq(1.0)) }
  end

  describe '#to_hex' do
    subject { color.to_hex }

    it { is_expected.to(eq('FF0001')) }
  end

  describe '#to_s' do
    subject { color.to_s }

    it { is_expected.to(eq('Color(r: FF, g: 00, b: 01)')) }
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

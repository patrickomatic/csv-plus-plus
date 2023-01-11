# frozen_string_literal: true

require 'expand'
require 'modifier'

describe ::CSVPlusPlus::Modifier do
  let(:modifier) { ::CSVPlusPlus::Modifier.new }

  describe 'border=' do
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

  describe 'format=' do
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

  describe 'expand=' do
    let(:amount) { nil }
    before { modifier.expand = expand }
    subject(:expand) { ::CSVPlusPlus::Expand.new(amount) }

    it { is_expected.to(be_infinite) }

    context 'with an amount' do
      let(:amount) { 2 }

      it { is_expected.not_to(be_infinite) }
    end
  end

  describe 'color=' do
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
end

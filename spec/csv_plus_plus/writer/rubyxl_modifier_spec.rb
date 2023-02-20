# frozen_string_literal: true

describe ::CSVPlusPlus::Writer::RubyXLModifier do
  let(:modifier) { build(:modifier) }
  let(:rubyxl_modifier) { described_class.new(modifier) }

  describe '#border_weight' do
    subject { rubyxl_modifier.border_weight }

    it { is_expected.to(eq('thin')) }
  end

  describe '#number_format_code' do
    subject { rubyxl_modifier.number_format_code }

    it { is_expected.to(be_nil) }

    context 'with a valid number format' do
      let(:modifier) { build(:modifier, numberformat: 'number') }

      it { is_expected.to(eq('0')) }
    end
  end
end

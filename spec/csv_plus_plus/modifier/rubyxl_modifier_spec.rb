# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Modifier::RubyXLModifier do
  let(:rubyxl_modifier) { described_class.new }

  describe '#border_weight' do
    subject { rubyxl_modifier.border_weight }

    it { is_expected.to(eq('thin')) }
  end

  describe '#number_format_code' do
    subject { rubyxl_modifier.number_format_code }

    it { is_expected.to(be_nil) }

    context 'with a valid number format' do
      before do
        rubyxl_modifier.numberformat = ::CSVPlusPlus::Modifier::NumberFormat::DateTime
      end

      it { is_expected.to(eq('m/d/yyyy h:mm')) }
    end
  end
end

# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::Date do
  subject(:entity) { described_class.new('25/4/2023') }

  describe '.valid_date?' do
    it 'is false for invalid dates' do
      expect(described_class.valid_date?('foo')).to(eq(false))
    end

    it 'is true for dates with full year' do
      expect(described_class.valid_date?('11/12/2023')).to(eq(true))
    end

    it 'is true for dates with 2-digit year' do
      expect(described_class.valid_date?('11/12/23')).to(eq(true))
    end
  end

  describe '#initialize' do
    it 'sets @type' do
      expect(subject.type).to(eq(::CSVPlusPlus::Entities::Type::Date))
    end

    it 'converts to a ::Date' do
      expect(subject.value).to(be_a(::Date))
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:date, value: '25/4/2023'))) }

    it { is_expected.not_to(eq(build(:boolean_false))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#evaluate' do
    let(:runtime) { build(:runtime) }

    subject { entity.evaluate(runtime) }

    it { is_expected.to(eq('04/25/23')) }
  end
end

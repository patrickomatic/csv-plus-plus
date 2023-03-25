# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::Number do
  let(:value) { '55' }

  subject(:entity) { described_class.new(value) }

  describe '#initialize' do
    it 'has a nil id' do
      expect(subject.id).to(be_nil)
    end

    it 'sets the value' do
      expect(subject.value).to(eq(55))
    end

    it 'converts the value to an Integer' do
      expect(subject.value).to(be_a(::Integer))
    end

    context 'with a float value' do
      let(:value) { '55.5' }

      it 'converts the value to a Float' do
        expect(subject.value).to(be_a(::Float))
      end
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:number, n: 55))) }

    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#evaluate' do
    let(:runtime) { build(:runtime) }

    subject { entity.evaluate(runtime) }

    it { is_expected.to(eq('55')) }
  end

  describe '#number?' do
    it { is_expected.to(be_number) }
  end
end

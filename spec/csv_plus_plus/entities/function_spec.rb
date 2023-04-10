# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::Function do
  subject(:entity) { described_class.new(:foo, %w[a b], build(:number_one)) }

  describe '#initialize' do
    it 'sets @type' do
      expect(subject.type).to(eq(::CSVPlusPlus::Entities::Type::Function))
    end

    it 'lowercases and converts the id to a symbol' do
      expect(subject.id).to(eq(:foo))
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:fn, name: :foo, arguments: %w[a b], body: build(:number_one)))) }

    it { is_expected.not_to(eq(build(:fn_foo))) }
    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#evaluate' do
    let(:runtime) { build(:runtime) }

    subject { entity.evaluate(runtime) }

    it { is_expected.to(eq('def FOO(a, b) 1')) }
  end
end

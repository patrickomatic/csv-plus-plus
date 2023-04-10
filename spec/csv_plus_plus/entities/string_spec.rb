# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::String do
  subject(:entity) { described_class.new('foo') }

  describe '#initialize' do
    it 'sets @type' do
      expect(entity.type).to(eq(::CSVPlusPlus::Entities::Type::String))
    end

    it 'has a nil id' do
      expect(entity.id).to(be_nil)
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:string, s: 'foo'))) }

    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#evaluate' do
    let(:runtime) { build(:runtime) }

    subject { entity.evaluate(runtime) }

    it { is_expected.to(eq('"foo"')) }
  end
end

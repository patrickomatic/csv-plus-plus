# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::String do
  subject(:entity) { described_class.new('foo') }

  describe '#==' do
    it { is_expected.to(eq(build(:string, s: 'foo'))) }

    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#evaluate' do
    let(:position) { build(:position) }

    subject { entity.evaluate(position) }

    it { is_expected.to(eq('"foo"')) }
  end
end

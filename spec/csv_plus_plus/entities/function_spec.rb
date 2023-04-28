# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::Function do
  subject(:entity) { described_class.new(:foo, %w[a b], build(:number_one)) }

  describe '#==' do
    it { is_expected.to(eq(build(:fn, name: :foo, arguments: %w[a b], body: build(:number_one)))) }

    it { is_expected.not_to(eq(build(:fn_foo))) }
    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#evaluate' do
    let(:position) { build(:position) }

    subject { entity.evaluate(position) }

    it { is_expected.to(eq('def FOO(a, b) 1')) }
  end
end

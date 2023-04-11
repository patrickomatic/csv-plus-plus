# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Runtime::Value do
  let(:resolve_fn) { ->(_r, _args) { build(:number_zero) } }

  subject(:entity) { described_class.new(resolve_fn) }

  describe '#call' do
    let(:runtime) { build(:runtime) }

    subject { entity.call(runtime, []) }

    it { is_expected.to(eq(build(:number_zero))) }
  end
end

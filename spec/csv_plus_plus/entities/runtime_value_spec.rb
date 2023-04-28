# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::RuntimeValue do
  let(:resolve_fn) { ->(_r, _args) { build(:number_zero) } }

  subject(:entity) { described_class.new(resolve_fn) }

  describe '#call' do
    let(:position) { build(:position) }

    subject { entity.call(position, []) }

    it { is_expected.to(eq(build(:number_zero))) }
  end
end

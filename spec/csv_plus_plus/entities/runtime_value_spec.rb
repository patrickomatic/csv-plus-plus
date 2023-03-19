# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::RuntimeValue do
  let(:resolve_fn) { -> { build(:number_zero) } }

  subject(:runtime_value) { described_class.new(resolve_fn) }

  describe '#initialize' do
    it 'has a nil id' do
      expect(subject.id).to(be_nil)
    end
  end

  describe '#runtime_value?' do
    it { is_expected.to(be_runtime_value) }
  end

  describe '#==' do
    it { is_expected.to(eq(build(:runtime_value, resolve_fn:))) }

    it { is_expected.not_to(eq(build(:fn_foo))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#to_s' do
    subject { runtime_value.to_s }

    it { is_expected.to(eq('(runtime_value)')) }
  end
end

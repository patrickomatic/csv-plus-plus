# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Runtime::CanDefineReferences do
  let(:functions) { {} }
  let(:variables) { {} }

  let(:runtime) { build(:runtime, variables:, functions:) }

  subject { runtime }

  describe '#def_variable' do
    let(:id) { 'foo' }
    let(:value) { build(:number_one) }

    before { runtime.def_variable(id, value) }

    it 'sets the values in @variables' do
      expect(runtime.variables).to(eq({ foo: build(:number_one) }))
    end
  end

  describe '#def_variables' do
    let(:var_foo) { build(:variable_foo) }
    let(:id) { 'foo' }

    before { runtime.def_variable(id, var_foo) }

    it 'sets the function in @variables' do
      expect(runtime.variables).to(eq({ foo: build(:variable_foo) }))
    end

    it 'overwrites previous definitions' do
      var_bar = build(:variable_bar)
      runtime.def_variable(:foo, var_bar)
      expect(runtime.variables).to(eq({ foo: build(:variable_bar) }))
    end
  end

  describe '#def_function' do
    let(:fn_foo) { build(:fn_foo) }
    let(:id) { 'foo' }

    before { runtime.def_function(id, fn_foo) }

    it 'sets the function in @functions' do
      expect(runtime.functions).to(eq({ foo: build(:fn_foo) }))
    end

    it 'overwrites previous definitions' do
      fn_bar = build(:fn_bar)
      runtime.def_function(:foo, fn_bar)
      expect(runtime.functions).to(eq({ foo: build(:fn_bar) }))
    end
  end

  describe '#defined_variable?' do
    let(:var_id) { :foo }
    let(:variables) { { foo: build(:number_one) } }

    it { is_expected.to(be_defined_variable(var_id)) }

    context 'with an undefined variable' do
      let(:var_id) { :bar }

      it { is_expected.not_to(be_defined_variable(var_id)) }
    end
  end

  describe '#defined_function?' do
    let(:fn_id) { :foo }
    let(:functions) { { foo: build(:fn_foo) } }

    it { is_expected.to(be_defined_function(fn_id)) }
  end

  describe '#verbose_summary' do
    subject { runtime.verbose_summary }

    it { is_expected.to(match(/\#.*Variables/)) }
    it { is_expected.to(match(/\#.*Functions/)) }
  end
end

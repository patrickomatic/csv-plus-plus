# frozen_string_literal: true

describe ::CSVPlusPlus::CodeSection do
  let(:functions) { {} }
  let(:variables) { {} }
  let(:code_section) { build(:code_section, variables:, functions:) }

  describe '#def_variable' do
    let(:id) { 'foo' }
    let(:value) { build(:number_one) }

    before { code_section.def_variable(id, value) }

    it 'sets the values in @variables' do
      expect(code_section.variables).to(eq({ foo: build(:number_one) }))
    end
  end

  describe '#def_variables' do
    let(:var_foo) { build(:variable_foo) }
    let(:id) { 'foo' }

    before { code_section.def_variable(id, var_foo) }

    it 'sets the function in @variables' do
      expect(code_section.variables).to(eq({ foo: build(:variable_foo) }))
    end

    it 'overwrites previous definitions' do
      var_bar = build(:variable_bar)
      code_section.def_variable(:foo, var_bar)
      expect(code_section.variables).to(eq({ foo: build(:variable_bar) }))
    end
  end

  describe '#def_function' do
    let(:fn_foo) { build(:fn_foo) }
    let(:id) { 'foo' }

    before { code_section.def_function(id, fn_foo) }

    it 'sets the function in @functions' do
      expect(code_section.functions).to(eq({ foo: build(:fn_foo) }))
    end

    it 'overwrites previous definitions' do
      fn_bar = build(:fn_bar)
      code_section.def_function(:foo, fn_bar)
      expect(code_section.functions).to(eq({ foo: build(:fn_bar) }))
    end
  end

  describe '#defined_variable?' do
    let(:var_id) { :foo }
    let(:variables) { { foo: build(:number_one) } }

    subject { code_section }

    it { is_expected.to(be_defined_variable(var_id)) }

    context 'with an undefined variable' do
      let(:var_id) { :bar }

      it { is_expected.not_to(be_defined_variable(var_id)) }
    end
  end

  describe '#defined_function?' do
    let(:fn_id) { :foo }
    let(:functions) { { foo: build(:fn_foo) } }

    subject { code_section }

    it { is_expected.to(be_defined_function(fn_id)) }
  end

  describe '#to_s' do
    subject { code_section.to_s }

    it { is_expected.to(eq('CodeSection(functions: {}, variables: {})')) }
  end
end

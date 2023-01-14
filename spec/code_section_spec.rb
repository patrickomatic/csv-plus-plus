# frozen_string_literal: true

require 'code_section'
require 'tempfile'

describe ::CSVPlusPlus::CodeSection do
  let(:code_section) { build(:code_section) }

  describe '#def_variable' do
    let(:id) { 'foo' }
    let(:value) { build(:number_one) }

    before { code_section.def_variable(id, value) }

    it 'sets the values in @variables' do
      expect(code_section.variables).to(eq({ foo: build(:number_one) }))
    end
  end

  describe '#def_variables' do
    it 'sets the values in @variables, overwriting existing variables' do
      # TODO
    end
  end

  describe '#def_function' do
    let(:fn_foo) { build(:fn_foo) }
    let(:id) { 'foo' }

    before { code_section.def_function(id, fn_foo.arguments, fn_foo.body) }

    it 'sets the function in @functions' do
      expect(code_section.functions).to(eq({ foo: build(:fn_foo) }))
    end

    it 'overwrites previous definitions' do
      fn_bar = build(:fn_bar)
      code_section.def_function(:foo, fn_bar.arguments, fn_bar.body)
      expect(code_section.functions).to(eq({ foo: build(:fn_bar) }))
    end
  end

  describe '#to_s' do
    subject { code_section.to_s }

    it { is_expected.to(eq('CodeSection(functions: {}, variables: {})')) }
  end
end

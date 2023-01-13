# frozen_string_literal: true

require 'code_section'
require 'tempfile'

describe ::CSVPlusPlus::CodeSection do
  describe '#def_variable' do
    let(:id) { 'foo' }
    let(:value) { build(:number_one) }
    let(:code_section) { build(:code_section) }

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
    # TODO
  end
end

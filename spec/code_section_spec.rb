# frozen_string_literal: true

require 'code_section'
require 'tempfile'

describe ::CSVPlusPlus::CodeSection do
  describe '::parse' do
    let(:ec) { build(:execution_context, input:) }

    subject { described_class.parse(ec) }

    context 'with no code section' do
      let(:input) { 'foo,bar,baz' }

      it { is_expected.not_to(be_nil) }

      it 'has empty variables' do
        expect(subject.variables).to(be_empty)
      end

      it 'has empty functions' do
        expect(subject.functions).to(be_empty)
      end
    end

    context 'with comments' do
      let(:input) { "# this is a comment\n---\nfoo,bar,bar" }

      it { is_expected.not_to(be_nil) }

      it 'has empty variables' do
        expect(subject.variables).to(be_empty)
      end

      it 'has empty functions' do
        expect(subject.functions).to(be_empty)
      end
    end

    context 'with variable definitions' do
      let(:input) { "foo := 1\n---\nfoo,bar,baz" }

      it { is_expected.not_to(be_nil) }

      it 'sets a variable' do
        expect(subject.variables).to(eq({ foo: build(:number_one) }))
      end

      it 'has empty functions' do
        expect(subject.functions).to(be_empty)
      end
    end

    context 'with function definitions' do
      # TODO
    end
  end

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

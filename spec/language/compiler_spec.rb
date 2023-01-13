# frozen_string_literal: true

require 'compiler'

describe ::CSVPlusPlus::Language::Compiler do
  describe '::parse' do
    let(:compiler) { build(:compiler, input:) }
    let(:key_values) { {} }

    subject { compiler.parse_code_section(key_values:) }

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
end

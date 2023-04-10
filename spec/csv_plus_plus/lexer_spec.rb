# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Lexer do
  describe '#unquote' do
    let(:str) { 'just a string' }

    subject { described_class.unquote(str) }

    it { is_expected.to(eq('just a string')) }

    context 'with quotes on the end' do
      let(:str) { "'string with quotes'" }

      it { is_expected.to(eq('string with quotes')) }
    end

    context 'with spaces on the end' do
      let(:str) { ' string with spaces  ' }

      it { is_expected.to(eq('string with spaces')) }
    end

    context 'with backslash escapes' do
      let(:str) { "string with \\' escapes" }

      it { is_expected.to(eq("string with ' escapes")) }
    end

    context 'with an escaped backslash' do
      let(:str) { 'string with \\\\ backslash' }

      it { is_expected.to(eq('string with \\ backslash')) }
    end
  end
end

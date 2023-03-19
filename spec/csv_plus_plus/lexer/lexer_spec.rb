# frozen_string_literal: true

class TestParser
  include ::CSVPlusPlus::Lexer
  attr_accessor :anything_to_parse

  def anything_to_parse?(_input)
    anything_to_parse
  end

  def parse_subject
    'bar'
  end

  def return_value
    'foo'
  end

  def tokenizer
    ::CSVPlusPlus::Lexer::Tokenizer.new(tokens: [[/\d+/, :number]])
  end

  def do_parse; end
end

class TestParserThatThrowsParseError < TestParser
  def do_parse
    raise(::Racc::ParseError)
  end
end

describe ::CSVPlusPlus::Lexer do
  describe '#initialize' do
    let(:lexer) { ::TestParser.new }

    it 'initializes with empty tokens' do
      expect(lexer.tokens).to(be_empty)
    end
  end

  describe '#next_token' do
    let(:lexer) { ::TestParser.new(tokens: [1, 2, 3]) }

    it 'shifts each token' do
      expect(lexer.next_token).to(eq(1))
      expect(lexer.next_token).to(eq(2))
      expect(lexer.next_token).to(eq(3))
      expect(lexer.next_token).to(be_nil)
    end
  end

  describe '#parse' do
    let(:lexer) { ::TestParser.new }
    let(:runtime) { build(:runtime) }
    let(:anything_to_parse) { false }
    let(:input) { nil }

    before { lexer.anything_to_parse = anything_to_parse }

    subject { lexer.parse(input, runtime) }

    context 'input is nil' do
      it { is_expected.to(be_nil) }
    end

    context 'when nothing to parse' do
      let(:input) { '1' }

      it { is_expected.to(eq('foo')) }
    end

    context 'with something to parse' do
      let(:input) { '1' }
      let(:anything_to_parse) { true }

      it { is_expected.to(eq('foo')) }
    end

    context 'with a syntax error' do
      let(:input) { 'abc' }
      let(:anything_to_parse) { true }

      it 'raises a syntax error' do
        expect { subject }
          .to(raise_error(::CSVPlusPlus::Error::SyntaxError))
      end
    end

    context 'when do_parse throws a Racc::ParseError' do
      let(:lexer) { ::TestParserThatThrowsParseError.new }
      let(:input) { '1' }
      let(:anything_to_parse) { true }

      it 'raises a syntax error' do
        expect { subject }
          .to(raise_error(::CSVPlusPlus::Error::SyntaxError))
      end
    end
  end
end

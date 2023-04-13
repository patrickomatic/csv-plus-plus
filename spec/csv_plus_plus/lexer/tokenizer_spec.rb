# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Lexer::Tokenizer do
  let(:input) { 'this is an input' }
  let(:alter_matches) { {} }
  let(:catchall) { nil }
  let(:ignore) { nil }
  let(:tokens) { [] }
  let(:stop_fn) { nil }

  subject(:tokenizer) { described_class.new(alter_matches:, catchall:, ignore:, tokens:, stop_fn:).scan(input) }

  describe '#initialize' do
    it 'sets initial state' do
      expect(subject.scanner).not_to(be_nil)
      expect(subject.last_match).to(be_nil)
    end
  end

  describe '#scan_catchall' do
    subject { tokenizer.scan_catchall }

    it { is_expected.to(be_nil) }

    context 'when catchall is set' do
      let(:catchall) { /\w+/ }

      it { is_expected.to(eq('this')) }
    end
  end

  describe '#scan_tokens!' do
    before { tokenizer.scan_tokens! }

    it 'is nil without any tokens to match' do
      expect(subject.last_token).to(be_nil)
      expect(subject.last_match).to(be_nil)
    end

    context 'when it matches a token' do
      let(:tokens) { [::CSVPlusPlus::Lexer::Token.new(regexp: /\bthis\b/, token: :THIS)] }

      it 'sets last_token and last_match' do
        expect(subject.last_token.token).to(eq(:THIS))
        expect(subject.last_match).to(eq('this'))
      end

      context 'and alter_matches is set' do
        let(:alter_matches) { { THIS: ->(s) { s.gsub!(/this/, 'foo') } } }

        it 'alters last_match' do
          expect(subject.last_match).to(eq('foo'))
        end
      end
    end
  end

  describe '#matches_ignore?' do
    subject { tokenizer.matches_ignore? }

    it { is_expected.to(be_nil) }

    context 'when catchall is set' do
      let(:ignore) { /^this.*/ }

      it { is_expected.to(eq(input)) }
    end
  end

  describe '#rest' do
    subject { tokenizer.rest }

    it { is_expected.to(eq(input)) }
  end

  describe '#stop?' do
    subject { tokenizer.stop? }

    it { is_expected.to(be(false)) }

    context 'when @stop_fn is set' do
      let(:stop_fn) { ->(_s) { true } }
      it { is_expected.to(be(true)) }
    end
  end

  describe ' #peek' do
    subject { tokenizer.peek }

    it { is_expected.to(eq(input)) }
  end
end

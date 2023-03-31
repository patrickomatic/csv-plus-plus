# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::CliFlag do
  let(:options) { build(:options) }

  describe '::SUPPORTED_CSVPP_FLAGS' do
    let(:v) { 'foo' }

    it 'handler can be called with (options, v)' do
      ::SUPPORTED_CSVPP_FLAGS.each do |flag|
        expect { flag.handler.call(options, v) }
          .not_to(raise_error)
      end
    end
  end

  describe '#to_s' do
    subject { described_class.new('-f', '--foo', 'Foo bar', ->(options, v) {}).to_s }

    it { is_expected.to(eq('-f, --foo  Foo bar')) }
  end
end

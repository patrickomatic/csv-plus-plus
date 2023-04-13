# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::CLIFlag do
  let(:options) { build(:options) }

  describe '::SUPPORTED_CSVPP_FLAGS' do
    let(:v) { 'foo' }

    it 'handler can be called with (options, v)' do
      ::CSVPlusPlus::SUPPORTED_CSVPP_FLAGS.each do |flag|
        expect { flag.handler.call(options, v) }
          .not_to(raise_error)
      end
    end
  end
end

# frozen_string_literal: true

describe ::CSVPlusPlus::CLI do
  describe '.launch_compiler' do
    # TODO
  end

  describe '#initialize' do
    let(:cli) { described_class.new }

    xit 'validates the CLI flags' do
      expect { cli }
        .to(raise_error(::CSVPlusPlus::Error))
    end
  end

  describe '#compile!' do
    # TODO
  end

  describe '#handle_error' do
    let(:cli) { described_class.new }
    let(:error) { ::CSVPlusPlus::Error.new('error') }

    subject { cli.handle_error(error) }

    xit { is_expected.not_to(raise_error) }
  end
end

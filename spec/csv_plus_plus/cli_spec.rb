# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::CLI do
  subject(:cli) { described_class.new }

  describe '#main' do
    # TODO
  end

  describe '#initialize' do
    let(:argv) { %w[csv++ --output foo.xls --verbose] }

    before { stub_const('ARGV', argv) }

    it 'validates the CLI flags' do
      subject
      expect(cli.options.output_filename).to(eq('foo.xls'))
      expect(cli.options.verbose).to(be(true))
    end

    context 'with the help flag' do
      # TODO: need to mock ::Kernel.exit
    end

    context 'with invalid option flags' do
      let(:argv) { %w[csv++ --foo --bar] }

      it 'raises an error' do
        expect { subject }
          .to(raise_error(::CSVPlusPlus::Error::CLIError))
      end
    end

    context 'without required option flags' do
      let(:argv) { %w[csv++] }

      it 'raises an error' do
        expect { subject }
          .to(raise_error(::CSVPlusPlus::Error::CLIError))
      end
    end
  end
end

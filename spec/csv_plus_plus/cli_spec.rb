# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::CLI do
  let(:cli) { described_class.new }

  describe '.launch_compiler' do
    # TODO
  end

  describe '#main' do
    # TODO
  end

  describe '#parse_options!' do
    let(:argv) { %w[csv++ --output foo.xls --verbose] }

    before { stub_const('ARGV', argv) }

    subject { cli.parse_options! }

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
          .to(raise_error(::CSVPlusPlus::Error::Error))
      end
    end

    context 'without required option flags' do
      let(:argv) { %w[csv++] }

      it 'raises an error' do
        expect { subject }
          .to(raise_error(::CSVPlusPlus::Error::Error))
      end
    end
  end

  describe '#handle_error' do
    let(:error) { ::CSVPlusPlus::Error::Error.new('error') }
    let(:options) { build(:options) }

    before do
      cli.options = options
      allow(self).to(receive(:warn))
    end

    subject { cli.handle_error(error) }

    it "prints but doesn't raise the error" do
      # TODO: mock and assert ::Kernel.warn is called
      expect { subject }
        .not_to(raise_error)
    end

    context 'with a syntax error' do
      let(:runtime) { build(:runtime) }
      let(:error) { ::CSVPlusPlus::Error::FormulaSyntaxError.new('error', 'syntax error', runtime) }
      let(:options) { build(:options, verbose: true) }

      it "prints but doesn't raise the error" do
        # TODO: mock and assert ::Kernel.warn is called
        expect { subject }
          .not_to(raise_error)
      end
    end

    context 'with a Google::Apis::ClientError' do
      let(:error) { ::Google::Apis::ClientError.new('google error') }

      it "prints but doesn't raise the error" do
        # TODO: mock and assert ::Kernel.warn is called
        expect { subject }
          .not_to(raise_error)
      end

      context 'when verbose' do
        let(:options) { build(:options, verbose: true) }

        it "prints but doesn't raise the error" do
          # TODO: mock and assert ::Kernel.warn is called
          expect { subject }
            .not_to(raise_error)
        end
      end
    end

    context 'with an unexpected error' do
      let(:error) { ::StandardError.new('unexpected error') }

      it "prints but doesn't raise the error" do
        # TODO: mock and assert ::Kernel.warn is called
        expect { subject }
          .not_to(raise_error)
      end
    end
  end

  describe '#to_s' do
    subject { cli.to_s }

    it { is_expected.to(eq('CLI(options: )')) }
  end
end

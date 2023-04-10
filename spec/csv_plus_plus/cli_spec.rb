# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::CLI do
  subject(:cli) { described_class.new }

  describe '.launch_compiler' do
    # TODO
  end

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
    let(:argv) { %w[csv++ --output foo.xls --verbose] }
    let(:error) { ::CSVPlusPlus::Error::Error.new('error') }

    before do
      stub_const('ARGV', argv)
      allow(self).to(receive(:warn))
    end

    subject { cli.handle_error(error) }

    it "prints but doesn't raise the error" do
      # TODO: mock and assert ::Kernel.warn is called
      expect { subject }
        .not_to(raise_error)
    end

    context 'with a syntax error' do
      let(:runtime) { build(:runtime, cell_index: 0) }
      let(:error) { ::CSVPlusPlus::Error::FormulaSyntaxError.new('error', 'syntax error', runtime) }

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
end

# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::ErrorFormatter do
  let(:options) { build(:options) }
  let(:row_index) { nil }
  let(:cell) { nil }
  let(:cell_index) { nil }
  let(:line_number) { 1 }
  let(:position) { build(:position, cell:, line_number:, row_index:, cell_index:) }
  let(:runtime) { build(:runtime, position:) }

  let(:error_formatter) { described_class.new(options:, runtime:) }

  describe '#handle_error' do
    let(:error) { ::CSVPlusPlus::Error::CLIError.new('error') }

    subject { error_formatter.handle_error(error) }

    it 'warns a message' do
      expect(error_formatter).to(receive(:warn).with('error'))
      subject
    end

    context 'with a FormulaSyntaxError' do
      let(:error) { ::CSVPlusPlus::Error::FormulaSyntaxError.new('You made a mistake', bad_input: 'foobar') }

      it 'warns an error' do
        expect(error_formatter).to(receive(:warn).with('test.csvpp:1 You made a mistake: "foobar"'))
        subject
      end

      context 'with a cell and row index' do
        let(:line_number) { 10 }
        let(:row_index) { 0 }
        let(:cell) { build(:cell, index: cell_index) }
        let(:cell_index) { 5 }

        it 'includes the line, row and cell index in the message' do
          expect(error_formatter).to(receive(:warn).with('test.csvpp:10[0,5] You made a mistake: "foobar"'))
          subject
        end
      end

      context 'with a #wrapped_error' do
        let(:error) do
          ::CSVPlusPlus::Error::FormulaSyntaxError.new(
            'Uh oh',
            bad_input: 'bad input',
            wrapped_error: ::StandardError.new('foo')
          )
        end

        it 'does not include the wrapped error by default' do
          expect(error_formatter).to(receive(:warn).with('test.csvpp:1 Uh oh: "bad input"'))
          subject
        end

        context 'with a #wrapped_error and verbose: true' do
          let(:options) { build(:options, verbose: true) }

          it 'includes the wrapped error message' do
            expect(error_formatter).to(receive(:warn).once.with('test.csvpp:1 Uh oh: "bad input"'))
            expect(error_formatter).to(receive(:warn).once.with(/full_message.*foo/))
            subject
          end
        end
      end
    end

    context 'with a Google::Apis::ClientError' do
      let(:error) { ::Google::Apis::ClientError.new('Google Error') }

      it 'handles google-specific errors' do
        expect(error_formatter).to(receive(:warn).with('Error making Google Sheets API request: Google Error'))
        subject
      end

      context 'when verbose' do
        let(:options) { build(:options, verbose: true) }

        it 'prints two warnings' do
          expect(error_formatter).to(receive(:warn).twice)
          subject
        end
      end
    end

    context 'with an unexpected error' do
      let(:error) { ::StandardError.new('Unexpected error') }

      it 'includes a message with details to debug' do
        expect(error_formatter).to(receive(:warn).with(/An unexpected error was encountered/))
        subject
      end
    end
  end
end

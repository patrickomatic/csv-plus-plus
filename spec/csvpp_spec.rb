# frozen_string_literal: true

require_relative '../lib/csvpp'

describe ::CSVPlusPlus do
  let(:filename) { 'foo.csvpp' }
  let(:input) do
    <<~INPUT
      var := 42
      def added(a, b, c) ADD(CELLREF(a), CELLREF(b), CELLREF(c))
      ---
      [[format=bold]]foo,"=ADD($$var, 22)",baz
      1,2,3,=ADDED(A, B, C)
    INPUT
  end

  describe '.apply_template_to_sheet!' do
    subject { described_class.apply_template_to_sheet!(input, filename, options) }

    context 'to CSV' do
      let(:output_filename) { 'bar.csv' }
      let(:options) { build(:options, output_filename:) }

      before { subject }

      after { ::File.delete(output_filename) if ::File.exist?(output_filename) }

      it 'creates the CSV' do
        expect(::File).to(exist(output_filename))
      end

      it 'parses the input and generates CSV' do
        expect(::File.read(output_filename)).to(eq(%{foo,"=ADD(42, 22)",baz\n}))
      end
    end

    context 'to Google Sheets', :vcr do
      # TODO
      # let(:google_sheet_id) { ::ENV.fetch('GOOGLE_SHEET_ID') }
      # let(:options) { build(:options, google_sheet_id:) }
    end

    context 'to OpenDocument' do
      # TODO
    end

    context 'to Excel' do
      # TODO
    end
  end
end

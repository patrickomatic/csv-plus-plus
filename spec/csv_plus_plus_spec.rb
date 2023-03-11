# frozen_string_literal: true

describe ::CSVPlusPlus do
  let(:backup) { false }
  let(:options) { build(:options, backup:, output_filename:) }
  let(:filename) { 'foo.csvpp' }
  let(:input) do
    <<~INPUT
      var := 42
      def added(a, b, c) SUM(CELLABOVE($$a), CELLADJACENT($$b), CELLBELOW($$c))
      def compute(a, b) ($$b - $$a) * 100
      ---
      [[format=bold]]foo,"=FOO($$var, 22)",baz
      1,2,3,"=ADDED(A, B, C)"
      "=COMPUTE(500, 400)"
    INPUT
  end

  describe '.apply_template_to_sheet!' do
    subject { described_class.apply_template_to_sheet!(input, filename, options) }

    context 'to CSV' do
      let(:output_filename) { 'bar.csv' }
      let(:backup) { false }
      let(:options) { build(:options, backup:, output_filename:) }

      after { ::File.delete(output_filename) if ::File.exist?(output_filename) }

      it 'creates the .csv file' do
        subject

        expect(::File).to(exist(output_filename))
      end

      it 'parses the input and generates CSV' do
        subject

        expect(::File.read(output_filename)).to(
          eq(<<~OUTPUT))
            foo,"=FOO(42, 22)",baz
            1,2,3,"=SUM(A1, B2, C3)"
            =((400 - 500) * 100)
          OUTPUT
      end

      context 'when options.backup is set' do
        let(:backup) { true }
        let(:original) { 'foo,bar,baz' }
        let(:backed_up_files) { ::Dir.children('.').grep(/bar-.+\.csv/) }

        before { ::File.write(output_filename, original) }

        it 'creates a backup file' do
          subject

          expect(backed_up_files.length).to(eq(1))
          ::File.delete(backed_up_files[0])
        end
      end
    end

    context 'to Google Sheets', :vcr do
      # TODO
    end

    context 'to Excel' do
      let(:output_filename) { 'bar.xlsx' }

      before { subject }

      after { ::File.delete(output_filename) if ::File.exist?(output_filename) }

      it 'creates the .xlsx file' do
        expect(::File).to(exist(output_filename))
      end
    end

    context 'to OpenDocument' do
      # TODO
    end
  end
end

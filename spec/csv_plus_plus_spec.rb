# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus do
  let(:backup) { false }
  let(:options) { build(:options, backup:, output_filename:) }
  let(:filename) { 'foo.csvpp' }
  let(:input) do
    <<~INPUT
      var := 42
      def added(a, b, c) SUM(CELLABOVE(a), CELLADJACENT(b), CELLBELOW(c))
      def compute(a, b) (b - a) * 100
      ---
      [[format=bold]]foo,"=var",baz
      1,2,3,"=ADDED(A, B, C)"
      "=COMPUTE(500, 400)"
    INPUT
  end

  describe '.cli_compile' do
    subject { described_class.cli_compile(input, filename, options) }

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
            foo,=42,baz
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

      context 'a template with var= directives in an expand=' do
        let(:input) do
          <<~INPUT
            bar := test + 1
            ---
            Foo,Bar,Baz
            ![[expand=3]][[var=test]],=test*5,=bar
          INPUT
        end

        it 'properly resolves variable references in the expanded rows' do
          subject

          expect(::File.read(output_filename)).to(
            eq(<<~OUTPUT))
              Foo,Bar,Baz
              ,=(A2 * 5),=(A2 + 1)
              ,=(A3 * 5),=(A3 + 1)
              ,=(A4 * 5),=(A4 + 1)
            OUTPUT
        end
      end

      context 'a template with spacing for readability' do
        let(:input) do
          <<~INPUT
            bar := 2 + 1
            ---
            Foo               , Bar      , Baz
            ![[format=bold]]  , =bar*5   , =bar
          INPUT
        end

        it 'properly parses and resolves variables' do
          subject

          expect(::File.read(output_filename)).to(
            eq(<<~OUTPUT))
              Foo,Bar,Baz
              ,=((2 + 1) * 5),=(2 + 1)
            OUTPUT
        end
      end

      context 'a full working example' do
        let(:input) do
          <<~INPUT
            commission_charge := 0.65 # the broker charges $0.65 a contract/share

            fees := commission_charge * celladjacent(D)
            profit := (celladjacent(B) * celladjacent(C)) - fees

            ---
            ![[format=bold/halign=center/freeze]]Date ,Purchase         ,Price  ,Quantity ,Profit     ,Fees
            ![[expand=2]]                             ,[[format=bold]]  ,       ,         ,"=profit"  ,"=fees"
          INPUT
        end

        it 'properly parses and resolves variables' do
          subject

          expect(::File.read(output_filename)).to(
            eq(<<~OUTPUT))
              Date,Purchase,Price,Quantity,Profit,Fees
              ,,,,=((B2 * C2) - (0.65 * D2)),=(0.65 * D2)
              ,,,,=((B3 * C3) - (0.65 * D3)),=(0.65 * D3)
            OUTPUT
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

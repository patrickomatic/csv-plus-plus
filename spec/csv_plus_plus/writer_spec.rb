# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Writer do
  describe '.writer' do
    let(:position) { build(:position) }

    subject { described_class.writer(options, position) }

    context 'when options.google.sheet_id is set' do
      let(:options) { build(:google_sheets_options, sheet_id: 'asdf') }
      it { is_expected.to(be_a(::CSVPlusPlus::Writer::GoogleSheets)) }
    end

    context 'when options.output_filename ends in .csv' do
      let(:options) { build(:file_options, output_filename: 'foo.csv') }
      it { is_expected.to(be_a(::CSVPlusPlus::Writer::CSV)) }
    end

    context 'when options.output_filename ends in .ods' do
      let(:options) { build(:file_options, output_filename: 'foo.ods') }
      it { is_expected.to(be_a(::CSVPlusPlus::Writer::OpenDocument)) }
    end

    context 'when options.output_filename ends in .xlsx' do
      let(:options) { build(:file_options, output_filename: 'foo.xlsx') }
      it { is_expected.to(be_a(::CSVPlusPlus::Writer::Excel)) }
    end

    context 'otherwise it raises an exception' do
      let(:options) { build(:file_options, output_filename: 'foo.xyz') }
      it 'raises an error' do
        expect { subject }
          .to(raise_error(::StandardError))
      end
    end
  end
end

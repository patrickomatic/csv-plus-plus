# frozen_string_literal: true

require 'google/apis/sheets_v4'

describe ::CSVPlusPlus::Writer::GoogleSheetBuilder do
  let(:current_sheet_values) do
    [
      ['foo', 'bar', 0],
      [1, 2, 4],
      ['=add(1, 2)', 'foo', 3]
    ]
  end
  let(:sheet_id) { '#id' }
  let(:rows) { [build(:row), build(:row), build(:row)] }

  subject(:google_sheet_builder) { described_class.new(current_sheet_values:, sheet_id:, rows:) }

  describe '#batch_update_spreadsheet_request' do
    before { subject.batch_update_spreadsheet_request }

    it { is_expected.not_to(be_nil) }
  end
end

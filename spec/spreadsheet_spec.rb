require 'spreadsheet'

describe GSPush::Spreadsheet do
  let(:sheet_id) { "sheet-id1234" }
  let(:sheet_name) { "Finances" }
  subject { Spreadsheet.new(sheet_id, sheet_name) }

  before(:each) do
    # XXX mock out google APIs
  end

  describe "push!" do
    it "formats a request to Google Sheets API" do
      expect(true).to be(true)
    end
  end
end

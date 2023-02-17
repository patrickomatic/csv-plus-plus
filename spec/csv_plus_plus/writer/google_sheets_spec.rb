# frozen_string_literal: true

# TODO: this should probably be more specific than allowing any paths
google_sheets_path_matcher =
  lambda do |_r1, _r2|
    true
  end

describe ::CSVPlusPlus::Writer::GoogleSheets do
  let(:writer) { described_class.new(options) }

  before do
    allow(::Google::Auth).to(receive(:get_application_default).and_return({}))
  end

  describe '#write' do
    let(:options) { build(:options, :with_google_sheet_id) }
    let(:template) { build(:template, rows:) }

    subject { writer.write(template) }

    describe 'modifiers', vcr: { match_requests_on: [google_sheets_path_matcher] } do
      context 'format=' do
        let(:rows) { [row] }
        let(:row) do
          build(
            :row,
            cells: [
              build(:cell, index: 0, value: 'foo', modifier: build(:modifier, format: 'bold')),
              build(:cell, index: 1, value: 'bar', modifier: build(:modifier, format: 'strikethrough')),
              build(:cell, index: 2, value: 'foo1', modifier: build(:modifier, format: 'underline')),
              build(:cell, index: 3, value: 'bar1', modifier: build(:modifier, format: 'italic'))
            ]
          )
        end

        it 'successfully writes the spreadsheet' do
          expect { subject }
            .not_to(raise_error)
        end
      end
    end
  end
end

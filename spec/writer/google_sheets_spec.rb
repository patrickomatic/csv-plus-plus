# frozen_string_literal: true

require_relative '../../lib/writer/google_sheets'

describe ::CSVPlusPlus::Writer::GoogleSheets do
  subject { described_class.new(options) }

  describe '#write' do
    let(:google_sheet_id) { ::ENV.fetch('GOOGLE_SHEET_ID') }
    let(:options) { build(:options, google_sheet_id:) }
    let(:template) { build(:template, rows:) }

    before { subject.write(template) }

    describe 'modifiers', :vcr do
      context 'format=' do
        let(:rows) { [row] }
        let(:row) do
          build(
            :row,
            index: 0,
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

    describe 'an API error from Google Sheets API', :vcr do
      let(:google_sheet_id) { 'this-does-not-exist' }
      let(:rows) { [row] }
      let(:row) { build(:row, index: 0, cells: [build(:cell)]) }

      it 'logs the error and does not raise it' do
        expect { subject }
          .not_to(raise_error)
      end
    end
  end
end

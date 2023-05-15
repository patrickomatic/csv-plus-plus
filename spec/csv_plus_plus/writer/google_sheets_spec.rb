# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Writer::GoogleSheets do
  let(:position) { build(:position) }
  let(:writer) { described_class.new(options, position) }

  before do
    # allow(::Google::Auth).to(receive(:get_application_default).and_return({}))
  end

  describe '#write' do
    let(:options) { build(:google_sheets_options, sheet_name: 'Test') }
    let(:template) { build(:template, rows:) }

    subject { writer.write(template) }

    # xdescribe 'modifiers', vcr: { match_requests_on: [::Helpers::GoogleSheets::PathMatcher] } do
    xdescribe 'modifiers' do
      context 'format=' do
        let(:rows) { [row] }
        let(:row) do
          build(
            :row,
            cells: [
              build(
                :cell,
                index: 0,
                value: 'foo',
                modifier: build(:modifier, options:, format: ::CSVPlusPlus::Modifier::TextFormat::Bold)
              ),
              build(
                :cell,
                index: 1,
                value: 'bar',
                modifier: build(:modifier, options:, format: ::CSVPlusPlus::Modifier::TextFormat::Strikethrough)
              ),
              build(
                :cell,
                index: 2,
                value: 'foo1',
                modifier: build(:modifier, options:, format: ::CSVPlusPlus::Modifier::TextFormat::Underline)
              ),
              build(
                :cell,
                index: 3,
                value: 'bar1',
                modifier: build(:modifier, options:, format: ::CSVPlusPlus::Modifier::TextFormat::Italic)
              )
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

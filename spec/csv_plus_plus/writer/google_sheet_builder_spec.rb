# typed: false
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
  let(:options) { build(:options, :with_google_sheet_id) }
  let(:sheet_id) { 1_028_392 }
  let(:rows) { [build(:row), build(:row), build(:row)] }
  let(:position) { build(:position) }

  subject(:google_sheet_builder) { described_class.new(current_sheet_values:, sheet_id:, rows:, position:) }

  describe '#batch_update_spreadsheet_request' do
    let(:first_row) { subject.requests[0].update_cells.rows[0].values }
    subject { google_sheet_builder.batch_update_spreadsheet_request }

    it { is_expected.not_to(be_nil) }

    describe 'alignments' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(
                :cell,
                modifier: build(:modifier, options:, halign: ::CSVPlusPlus::Modifier::HorizontalAlign::Left)
              ),
              build(
                :cell,
                modifier: build(:modifier, options:, halign: ::CSVPlusPlus::Modifier::HorizontalAlign::Center)
              ),
              build(
                :cell,
                modifier: build(:modifier, options:, halign: ::CSVPlusPlus::Modifier::HorizontalAlign::Right)
              ),
              build(:cell, modifier: build(:modifier, options:, valign: ::CSVPlusPlus::Modifier::VerticalAlign::Top)),
              build(
                :cell,
                modifier: build(:modifier, options:, valign: ::CSVPlusPlus::Modifier::VerticalAlign::Center)
              ),
              build(
                :cell,
                modifier: build(:modifier, options:, valign: ::CSVPlusPlus::Modifier::VerticalAlign::Bottom)
              )
            ]
          )
        ]
      end

      it 'sets the alignments' do
        expect(first_row[0].user_entered_format.horizontal_alignment).to(eq('LEFT'))
        expect(first_row[1].user_entered_format.horizontal_alignment).to(eq('CENTER'))
        expect(first_row[2].user_entered_format.horizontal_alignment).to(eq('RIGHT'))
        expect(first_row[3].user_entered_format.vertical_alignment).to(eq('TOP'))
        expect(first_row[4].user_entered_format.vertical_alignment).to(eq('CENTER'))
        expect(first_row[5].user_entered_format.vertical_alignment).to(eq('BOTTOM'))
      end
    end

    describe 'borders' do
      let(:border_request) { subject.requests[1] }
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, options:, border: ::CSVPlusPlus::Modifier::BorderSide::Top)),
              build(:cell, modifier: build(:modifier, options:, border: ::CSVPlusPlus::Modifier::BorderSide::Left)),
              build(:cell, modifier: build(:modifier, options:, border: ::CSVPlusPlus::Modifier::BorderSide::Right)),
              build(:cell, modifier: build(:modifier, options:, border: ::CSVPlusPlus::Modifier::BorderSide::Bottom)),
              build(:cell, modifier: build(:modifier, options:, border: ::CSVPlusPlus::Modifier::BorderSide::All))
            ]
          )
        ]
      end

      it 'sets the borders' do
        expect(subject.requests[1].update_borders.top.style).to(eq('SOLID'))
        expect(subject.requests[1].update_borders.top.color).to(be_a(::Google::Apis::SheetsV4::Color))
        expect(subject.requests[2].update_borders.left.style).to(eq('SOLID'))
        expect(subject.requests[3].update_borders.right.style).to(eq('SOLID'))
        expect(subject.requests[4].update_borders.bottom.style).to(eq('SOLID'))

        expect(subject.requests[5].update_borders.top.style).to(eq('SOLID'))
        expect(subject.requests[5].update_borders.left.style).to(eq('SOLID'))
        expect(subject.requests[5].update_borders.right.style).to(eq('SOLID'))
        expect(subject.requests[5].update_borders.bottom.style).to(eq('SOLID'))
      end

      # TODO: test weights
    end

    describe 'colors' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, options:, color: ::CSVPlusPlus::Color.new('#FF00FF'))),
              build(:cell, modifier: build(:modifier, options:, fontcolor: ::CSVPlusPlus::Color.new('#00FFFF')))
            ]
          )
        ]
      end

      it 'sets the colors' do
        # TODO
      end
    end

    describe 'fonts' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, options:, fontfamily: 'Helvetica')),
              build(:cell, modifier: build(:modifier, options:, fontsize: 40))
            ]
          )
        ]
      end

      it 'sets the fonts' do
        expect(first_row[0].user_entered_format.text_format.font_family).to(eq('Helvetica'))
        expect(first_row[1].user_entered_format.text_format.font_size).to(eq(40))
      end
    end

    describe 'formats' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, options:, format: ::CSVPlusPlus::Modifier::TextFormat::Bold)),
              build(:cell, modifier: build(:modifier, options:, format: ::CSVPlusPlus::Modifier::TextFormat::Italic)),
              build(
                :cell,
                modifier: build(:modifier, options:, format: ::CSVPlusPlus::Modifier::TextFormat::Strikethrough)
              ),
              build(:cell, modifier: build(:modifier, options:, format: ::CSVPlusPlus::Modifier::TextFormat::Underline))
            ]
          )
        ]
      end

      it 'sets the formats' do
        expect(first_row[0].user_entered_format.text_format.bold).to(eq(true))
        expect(first_row[1].user_entered_format.text_format.italic).to(eq(true))
        expect(first_row[2].user_entered_format.text_format.strikethrough).to(eq(true))
        expect(first_row[3].user_entered_format.text_format.underline).to(eq(true))
      end
    end

    describe 'number formats' do
      # TODO
    end
  end
end

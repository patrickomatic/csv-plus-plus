# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Writer::RubyXLBuilder do
  let(:position) { build(:position) }
  let(:sheet_name) { 'Test Excel Sheet' }
  let(:rows) { [] }
  let(:output_filename) { ::Pathname.new('test.xlsx') }

  subject(:rubyxl_builder) { described_class.new(output_filename:, position:, rows:, sheet_name:) }

  describe '#build_workbook' do
    let(:worksheet) { subject.worksheets[0] }
    let(:first_row) { worksheet.sheet_data[0] }

    subject { rubyxl_builder.build_workbook }

    it 'sets the sheet name' do
      expect(worksheet.sheet_name).to(eq(sheet_name))
    end

    describe 'cell contents' do
      let(:rows) do
        [
          build(:row, cells: [build(:cell, value: 'test'), build(:cell, value: 'foo', ast: build(:number, n: 42))])
        ]
      end

      it 'sets cell values and formulas' do
        expect(first_row[0].value).to(eq('test'))
        expect(first_row[1].formula.expression).to(eq('42'))
      end
    end

    describe 'alignments' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, halign: ::CSVPlusPlus::Modifier::HorizontalAlign::Left)),
              build(:cell, modifier: build(:modifier, halign: ::CSVPlusPlus::Modifier::HorizontalAlign::Center)),
              build(:cell, modifier: build(:modifier, halign: ::CSVPlusPlus::Modifier::HorizontalAlign::Right)),
              build(:cell, modifier: build(:modifier, valign: ::CSVPlusPlus::Modifier::VerticalAlign::Top)),
              build(:cell, modifier: build(:modifier, valign: ::CSVPlusPlus::Modifier::VerticalAlign::Center)),
              build(:cell, modifier: build(:modifier, valign: ::CSVPlusPlus::Modifier::VerticalAlign::Bottom))
            ]
          )
        ]
      end

      it 'sets the alignments' do
        expect(first_row[0].horizontal_alignment).to(eq('left'))
        expect(first_row[1].horizontal_alignment).to(eq('center'))
        expect(first_row[2].horizontal_alignment).to(eq('right'))
        expect(first_row[3].vertical_alignment).to(eq('top'))
        expect(first_row[4].vertical_alignment).to(eq('center'))
        expect(first_row[5].vertical_alignment).to(eq('bottom'))
      end
    end

    describe 'borders' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, border: ::CSVPlusPlus::Modifier::BorderSide::Top)),
              build(:cell, modifier: build(:modifier, border: ::CSVPlusPlus::Modifier::BorderSide::Left)),
              build(:cell, modifier: build(:modifier, border: ::CSVPlusPlus::Modifier::BorderSide::Right)),
              build(:cell, modifier: build(:modifier, border: ::CSVPlusPlus::Modifier::BorderSide::Bottom)),
              build(:cell, modifier: build(:modifier, border: ::CSVPlusPlus::Modifier::BorderSide::All))
            ]
          )
        ]
      end

      it 'sets the borders' do
        expect(first_row[0].get_border('top')).to(eq('thin'))
        expect(first_row[1].get_border('left')).to(eq('thin'))
        expect(first_row[2].get_border('right')).to(eq('thin'))
        expect(first_row[3].get_border('bottom')).to(eq('thin'))
        # we set 'all' on the last one:
        expect(first_row[4].get_border('top')).to(eq('thin'))
        expect(first_row[4].get_border('left')).to(eq('thin'))
        expect(first_row[4].get_border('right')).to(eq('thin'))
        expect(first_row[4].get_border('bottom')).to(eq('thin'))
      end
    end

    describe 'colors' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, color: ::CSVPlusPlus::Color.new('#FF00FF'))),
              build(:cell, modifier: build(:modifier, fontcolor: ::CSVPlusPlus::Color.new('#00FFAA')))
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
              build(:cell, modifier: build(:modifier, fontfamily: 'Helvetica')),
              build(:cell, modifier: build(:modifier, fontsize: 40))
            ]
          )
        ]
      end

      it 'sets the fonts' do
        # TODO
      end
    end

    describe 'formats' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, format: ::CSVPlusPlus::Modifier::TextFormat::Bold)),
              build(:cell, modifier: build(:modifier, format: ::CSVPlusPlus::Modifier::TextFormat::Italic)),
              build(:cell, modifier: build(:modifier, format: ::CSVPlusPlus::Modifier::TextFormat::Strikethrough)),
              build(:cell, modifier: build(:modifier, format: ::CSVPlusPlus::Modifier::TextFormat::Underline))
            ]
          )
        ]
      end

      it 'sets the formats' do
        expect(first_row[0].get_cell_font.b.val).to(be(true))
        expect(first_row[1].get_cell_font.i.val).to(be(true))
        expect(first_row[2].get_cell_font.strike.val).to(be(true))
        expect(first_row[3].get_cell_font.u.val).to(be(true))
      end
    end

    describe 'number formats' do
      # TODO
    end
  end
end

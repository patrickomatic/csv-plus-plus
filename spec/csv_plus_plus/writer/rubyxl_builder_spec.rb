# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Writer::RubyXLBuilder do
  let(:runtime) { build(:runtime) }
  let(:sheet_name) { 'Test Excel Sheet' }
  let(:rows) { [] }
  let(:input_filename) { 'test.xlsx' }

  subject(:rubyxl_builder) { described_class.new(input_filename:, runtime:, rows:, sheet_name:) }

  describe '#build_workbook' do
    let(:worksheet) { subject.worksheets[0] }
    let(:first_row) { worksheet.sheet_data[0] }

    subject { rubyxl_builder.build_workbook }

    it 'sets the sheet name' do
      expect(worksheet.sheet_name).to(eq(sheet_name))
    end

    describe 'alignments' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, halign: 'left')),
              build(:cell, modifier: build(:modifier, halign: 'center')),
              build(:cell, modifier: build(:modifier, halign: 'right')),
              build(:cell, modifier: build(:modifier, valign: 'top')),
              build(:cell, modifier: build(:modifier, valign: 'center')),
              build(:cell, modifier: build(:modifier, valign: 'bottom'))
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
              build(:cell, modifier: build(:modifier, border: 'top')),
              build(:cell, modifier: build(:modifier, border: 'left')),
              build(:cell, modifier: build(:modifier, border: 'right')),
              build(:cell, modifier: build(:modifier, border: 'bottom')),
              build(:cell, modifier: build(:modifier, border: 'all'))
            ]
          )
        ]
      end

      it 'sets the borders' do
        # TODO
      end

      # TODO: test weights
    end

    describe 'colors' do
      let(:rows) do
        [
          build(
            :row,
            cells: [
              build(:cell, modifier: build(:modifier, color: '#FF00FF')),
              build(:cell, modifier: build(:modifier, fontcolor: '#00FFFF'))
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
              build(:cell, modifier: build(:modifier, format: 'bold')),
              build(:cell, modifier: build(:modifier, format: 'italic')),
              build(:cell, modifier: build(:modifier, format: 'strikethrough')),
              build(:cell, modifier: build(:modifier, format: 'underline'))
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

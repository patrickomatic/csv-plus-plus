# frozen_string_literal: true

require 'syntax_error'
require 'template'

describe ::CSVPlusPlus::Template do
  describe '#expand_rows!' do
    let(:template) { build(:template, rows:) }
    let(:cells_row0) do
      [
        build(:cell, row_index: 0, index: 0, value: 'foo'),
        build(:cell, row_index: 0, index: 1, value: 'foo'),
        build(:cell, row_index: 0, index: 2, value: 'foo')
      ]
    end
    let(:cells_row1) do
      [
        build(:cell, row_index: 1, index: 0, value: 'a'),
        build(:cell, row_index: 1, index: 1, value: 'nother'),
        build(:cell, row_index: 1, index: 2, value: 'row')
      ]
    end
    let(:rows) do
      [
        build(:row, index: 0, cells: cells_row0, modifier: build(:modifier_with_expand, repetitions: 2)),
        build(:row, index: 1, cells: cells_row1)
      ]
    end

    before { template.expand_rows! }

    it 'updates row.index' do
      expect(template.rows[0].index).to(eq(0))
      expect(template.rows[0].cells[0].row_index).to(eq(0))
      expect(template.rows[0].cells[1].row_index).to(eq(0))
      expect(template.rows[0].cells[2].row_index).to(eq(0))
      expect(template.rows[1].index).to(eq(1))
      expect(template.rows[1].cells[0].row_index).to(eq(1))
      expect(template.rows[1].cells[1].row_index).to(eq(1))
      expect(template.rows[1].cells[2].row_index).to(eq(1))
      expect(template.rows[2].index).to(eq(2))
      expect(template.rows[2].cells[0].row_index).to(eq(2))
      expect(template.rows[2].cells[1].row_index).to(eq(2))
      expect(template.rows[2].cells[2].row_index).to(eq(2))
    end

    context 'with an infinite expand' do
      let(:rows) do
        [
          build(:row, index: 0, cells: cells_row1),
          build(:row, index: 1, cells: cells_row0, modifier: build(:modifier_with_expand))
        ]
      end

      it 'expands the rows to SPREADSHEET_INFINITY' do
        expect(template.rows.length).to(eq(1000))
      end
    end

    context 'with an infinite expand that expands over top of rows after it' do
      let(:rows) do
        [
          build(:row, index: 0, cells: cells_row0, modifier: build(:modifier_with_expand)),
          build(:row, index: 1, cells: cells_row1),
          build(:row, index: 2, cells: cells_row1)
        ]
      end

      xit 'expands the rows to SPREADSHEET_INFINITY' do
        expect(template.rows.length).to(eq(1000))
      end
    end
  end
end

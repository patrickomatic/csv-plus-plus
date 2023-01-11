# frozen_string_literal: true

require 'syntax_error'
require 'template'

describe ::CSVPlusPlus::Template do
  let(:ec) { build(:execution_context, input:) }
  let(:input) { '' }

  describe '::run' do
    let(:template) { described_class.run(execution_context: ec) }
    let(:input) { "foo0,bar0,baz0\nfoo1,bar1,baz1\nfoo2,bar2,baz2\n" }

    it 'creates rows' do
      expect(template.rows.length).to(eq(3))
    end

    it 'sets row.index' do
      expect(template.rows[0].index).to(eq(0))
      expect(template.rows[1].index).to(eq(1))
      expect(template.rows[2].index).to(eq(2))
    end

    context 'with cell modifiers' do
      let(:input) { 'foo,[[align=right/format=bold]]bar,baz' }

      it 'creates cells with the modifiers' do
        expect(template.rows[0].cells[1].modifier).to(be_aligned('right'))
        expect(template.rows[0].cells[1].modifier).to(be_formatted('bold'))
      end
    end

    context 'with cell modifiers with multiple values' do
      let(:input) { 'foo,[[align=right/format=bold/format=italic]]bar,baz' }

      it 'creates cells with the modifiers' do
        expect(template.rows[0].cells[1].modifier).to(be_formatted('bold'))
        expect(template.rows[0].cells[1].modifier).to(be_formatted('italic'))
      end
    end

    context 'with row modifiers' do
      let(:input) { '![[align=center/format=bold]]foo,bar,baz' }

      it 'creates rows with the modifiers' do
        expect(template.rows[0].modifier).to(be_aligned('center'))
        expect(template.rows[0].modifier).to(be_formatted('bold'))
      end
    end
  end

  describe '#parse_csv_rows!' do
    let(:ec) { build(:execution_context, input:) }
    let(:template) { described_class.new(execution_context: ec) }
    let(:input) { "foo,bar,baz\nfoo1,bar1,baz1\nfoo2,bar2,baz2\n" }

    it 'parses the CSV rows' do
      template.parse_csv_rows!
      expect(template.rows.length).to(eq(3))
    end

    context 'with multiple infinite expands' do
      let(:input) { "![[expand]]foo,bar,baz\n![[expand]]foo1,bar1,baz1\nfoo2,bar2,baz2\n" }

      it 'throws a SyntaxError' do
        expect { template.parse_csv_rows! }
          .to(raise_error(::CSVPlusPlus::Language::SyntaxError))
      end
    end
  end

  describe '#expand_rows!' do
    let(:template) { described_class.new(rows:, execution_context: ec) }
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

  describe '#resolve_variables!' do
    let(:template) { build(:template, rows:, code_section:, execution_context: ec) }
    let(:code_section) { build(:code_section, variables:) }
    let(:cells_row0) do
      [
        build(:cell, row_index: 0, index: 0, value: '=$$foo', ast: build(:variable, id: :foo)),
        build(:cell, row_index: 0, index: 1, value: '=foo'),
        build(:cell, row_index: 0, index: 2, value: 'foo'),
        build(:cell, row_index: 0, index: 3, value: '=$$rownum', ast: build(:variable, id: :rownum))
      ]
    end
    let(:rows) { [build(:row, index: 0, cells: cells_row0)] }
    let(:variables) { { foo: build(:number_one) } }

    before { template.resolve_variables! }

    it 'resolves the first one and leaves the others alone' do
      expect(template.rows[0].cells[0].to_csv).to(eq('=1'))
      expect(template.rows[0].cells[1].to_csv).to(eq('=foo'))
      expect(template.rows[0].cells[2].to_csv).to(eq('foo'))
    end

    it 'resolves runtime variables' do
      expect(template.rows[0].cells[3].to_csv).to(eq('=1'))
    end

    context 'with key_values' do
      let(:key_values) { { rownum: '1111' } }

      xit 'resolves and overrides $$rownum' do
        expect(template.rows[0].cells[3].to_csv).to(eq('=1111'))
      end
    end
  end
end

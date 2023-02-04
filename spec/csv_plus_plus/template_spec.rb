# frozen_string_literal: true

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
          build(:row, index: 1, cells: cells_row0, modifier: build(:modifier_with_infinite_expand))
        ]
      end

      it 'expands the rows to SPREADSHEET_INFINITY' do
        expect(template.rows.length).to(eq(1000))
      end
    end
  end

  describe '#to_s' do
    subject { build(:template).to_s }

    it do
      is_expected.to(
        eq(
          'Template(rows: [], scope: Scope(code_section: CodeSection(functions: {}, variables: {}), ' \
          'runtime: Runtime(cell: , row_index: 0, cell_index: )))'
        )
      )
    end
  end

  describe '#validate_infinite_expands' do
    let(:template) { build(:template, rows:) }
    let(:rows) { [build(:row, index: 0, cells:)] }
    let(:cells) do
      [
        build(:cell, row_index: 0, index: 0, value: 'foo'),
        build(:cell, row_index: 0, index: 1, value: 'foo'),
        build(:cell, row_index: 0, index: 2, value: 'foo')
      ]
    end
    let(:runtime) { build(:runtime) }

    subject { template.validate_infinite_expands(runtime) }

    it 'does not raise an exception' do
      expect { subject }
        .not_to(raise_error)
    end

    context 'with a single bounded expand' do
      let(:rows) do
        [
          build(:row, index: 0, cells:, modifier: build(:modifier_with_expand)),
          build(:row, index: 1, cells:),
          build(:row, index: 2, cells:)
        ]
      end

      it 'does not raise an exception' do
        expect { subject }
          .not_to(raise_error)
      end
    end

    context 'with multiple bounded expands' do
      let(:rows) do
        [
          build(:row, index: 0, cells:, modifier: build(:modifier_with_expand)),
          build(:row, index: 1, cells:, modifier: build(:modifier_with_expand)),
          build(:row, index: 2, cells:)
        ]
      end

      it 'does not raise an exception' do
        expect { subject }
          .not_to(raise_error)
      end
    end

    context 'with one infinite expand' do
      let(:rows) do
        [
          build(:row, index: 0, cells:, modifier: build(:modifier_with_infinite_expand)),
          build(:row, index: 1, cells:),
          build(:row, index: 2, cells:)
        ]
      end

      it 'does not raise an exception' do
        expect { subject }
          .not_to(raise_error)
      end
    end

    context 'with multiple infinite expands' do
      let(:rows) do
        [
          build(:row, index: 0, cells:, modifier: build(:modifier_with_infinite_expand)),
          build(:row, index: 1, cells:, modifier: build(:modifier_with_infinite_expand)),
          build(:row, index: 2, cells:)
        ]
      end

      it 'does not raise an exception' do
        expect { subject }
          .to(raise_error(::CSVPlusPlus::Language::SyntaxError))
      end
    end
  end
end

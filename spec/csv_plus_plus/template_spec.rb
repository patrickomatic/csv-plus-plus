# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Template do
  let(:modifier_with_expand) { build(:modifier, expand: build(:expand, repetitions: 2)) }
  let(:modifier_with_infinite_expand) { build(:modifier, expand: build(:expand)) }
  let(:template) { build(:template, rows:) }

  describe '#bind_all_vars!' do
    before { template.bind_all_vars! }

    xit 'does something' do
      # XXX
    end
  end

  describe '#expand_rows!' do
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
        build(:row, index: 0, cells: cells_row0, modifier: modifier_with_expand),
        build(:row, index: 1, cells: cells_row1)
      ]
    end

    before { template.expand_rows! }

    it 'expands the rows' do
      expect(template.rows.length).to(eq(3))
    end

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
          build(:row, index: 1, cells: cells_row0, modifier: modifier_with_infinite_expand)
        ]
      end

      it 'expands the rows to SPREADSHEET_INFINITY' do
        expect(template.rows.length).to(eq(1000))
      end
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

    subject { template.validate_infinite_expands }

    it 'does not raise an exception' do
      expect { subject }
        .not_to(raise_error)
    end

    context 'with a single bounded expand' do
      let(:rows) do
        [
          build(:row, index: 0, cells:, modifier: modifier_with_expand),
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
          build(:row, index: 0, cells:, modifier: modifier_with_expand),
          build(:row, index: 1, cells:, modifier: modifier_with_expand),
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
          build(:row, index: 0, cells:, modifier: modifier_with_infinite_expand),
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
          build(:row, index: 0, cells:, modifier: modifier_with_infinite_expand),
          build(:row, index: 1, cells:, modifier: modifier_with_infinite_expand),
          build(:row, index: 2, cells:)
        ]
      end

      it 'raises an exception' do
        expect { subject }
          .to(raise_error(::CSVPlusPlus::Error::ModifierSyntaxError))
      end
    end
  end

  describe '#verbose_summary' do
    let(:rows) { [] }

    subject { template.verbose_summary }

    it { is_expected.to(match(/0 rows to be written/)) }
  end
end

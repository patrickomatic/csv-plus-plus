# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Runtime::PositionTracker do
  let(:runtime) { build(:runtime) }

  describe '#cell' do
    let(:runtime) { build(:runtime, cell: nil) }
    subject { runtime.cell }

    it 'raises an error when uninitialized' do
      expect { subject }
        .to(raise_error(::TypeError))
    end

    context 'when initialized' do
      let(:cell) { build(:cell) }

      before { runtime.cell = cell }

      it { is_expected.to(eq(cell)) }
    end
  end

  describe '#cell_index' do
    let(:runtime) { build(:runtime, cell_index: nil) }
    subject { runtime.cell_index }

    it 'raises an error when uninitialized' do
      expect { subject }
        .to(raise_error(::TypeError))
    end

    context 'when initialized' do
      let(:cell_index) { 0 }

      before { runtime.cell_index = cell_index }

      it { is_expected.to(eq(cell_index)) }
    end
  end

  describe '#line_number' do
    let(:runtime) { build(:runtime, line_number: nil) }
    subject { runtime.line_number }

    it 'raises an error when uninitialized' do
      expect { subject }
        .to(raise_error(::TypeError))
    end

    context 'when initialized' do
      let(:line_number) { 1 }

      before { runtime.line_number = line_number }

      it { is_expected.to(eq(line_number)) }
    end
  end

  describe '#map_lines' do
    let(:runtime) { build(:runtime, row_index:) }
    let(:row_index) { 0 }
    let(:lines) { %w[foo bar baz] }

    it 'emits each line' do
      expect { |block| runtime.map_lines(lines, &block) }
        .to(yield_successive_args(*lines))
    end

    it 'increments line_number each call' do
      expect(runtime.map_lines(lines) { runtime.line_number }).to(eq([1, 2, 3]))
    end

    it 'increments row_index if it is set' do
      expect(runtime.map_lines(lines) { runtime.row_index }).to(eq([0, 1, 2]))
    end

    context 'with a nil row_index' do
      let(:row_index) { nil }

      it 'raises an error when uninitialized' do
        expect { runtime.map_lines(lines) }
          .to(raise_error(::TypeError))
      end
    end
  end

  describe '#map_row' do
    let(:row) do
      [build(:cell, value: 'cell1'), build(:cell, value: 'cell2'), build(:cell, value: 'cell3')]
    end

    it 'emits each cell and index' do
      expect { |block| runtime.map_row(row, &block) }
        .to(yield_successive_args([::CSVPlusPlus::Cell, 0], [::CSVPlusPlus::Cell, 1], [::CSVPlusPlus::Cell, 2]))
    end

    it 'increments cell_index each call' do
      expect(runtime.map_row(row) { runtime.cell_index }).to(eq([0, 1, 2]))
    end

    it 'sets cell each call' do
      expect(runtime.map_row(row) { runtime.cell }).to(eq(row))
    end

    it 'does not increment line_number' do
      expect(runtime.map_row(row) { runtime.line_number }).to(eq([1, 1, 1]))
    end

    it 'does not increment row_index' do
      expect(runtime.map_row(row) { runtime.row_index }).to(eq([0, 0, 0]))
    end
  end

  describe '#map_rows' do
    let(:rows) do
      [
        %w[foo bar baz],
        %w[foo1 bar1 baz1],
        %w[foo2 bar2 baz2]
      ]
    end

    it 'iterates through each row' do
      expect { |block| runtime.map_rows(rows, &block) }
        .to(yield_successive_args(*rows))
    end

    it 'increments line_number each call' do
      expect(runtime.map_rows(rows) { runtime.line_number }).to(eq([1, 2, 3]))
    end

    it 'increments row_index each call' do
      expect(runtime.map_rows(rows) { runtime.row_index }).to(eq([0, 1, 2]))
    end
  end

  describe '#map_all_cells' do
    let(:rows) do
      [
        build(:row, cells: [build(:cell, value: 'foo')]),
        build(:row, cells: [build(:cell, value: 'foo1')]),
        build(:row, cells: [build(:cell, value: 'foo2')])
      ]
    end

    it 'iterates through all the cells' do
      expect { |block| runtime.map_all_cells(rows, &block) }
        .to(
          yield_successive_args(
            [::CSVPlusPlus::Cell, 0],
            [::CSVPlusPlus::Cell, 0],
            [::CSVPlusPlus::Cell, 0]
          )
        )
    end
  end

  describe '#row_index' do
    let(:runtime) { build(:runtime, row_index: nil) }

    subject { runtime.row_index }

    it 'raises an error when uninitialized' do
      expect { subject }
        .to(raise_error(::TypeError))
    end

    context 'when initialized' do
      let(:row_index) { 1 }

      before { runtime.row_index = row_index }

      it { is_expected.to(eq(row_index)) }
    end
  end

  describe '#rownum' do
    subject { runtime.rownum }

    it { is_expected.to(eq(1)) }

    context 'when @row_index is not set' do
      let(:runtime) { build(:runtime, row_index: nil) }

      it 'raises an error' do
        expect { subject }
          .to(raise_error(::TypeError))
      end
    end
  end
end

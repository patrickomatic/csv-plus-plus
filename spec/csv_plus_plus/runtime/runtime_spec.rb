# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Runtime::Runtime do
  let(:row_index) { 0 }
  let(:cell_index) { 0 }
  let(:variables) { {} }
  let(:runtime) { build(:runtime, cell_index:, row_index:, variables:) }

  describe '#initialize' do
    let(:filename) { 'foo.csvpp' }
    let(:input) do
      "
foo := 1
def function(a, b) add(a, b)
# a comment
---
foo,bar,baz
foo1,bar1,baz1
"
    end

    subject { described_class.new(input:, filename:) }

    it 'sets the filename' do
      expect(subject.filename).to(eq(filename))
    end

    context 'with a nil filename' do
      let(:filename) { nil }

      it 'filename defaults to "stdin"' do
        expect(subject.filename).to(eq('stdin'))
      end
    end

    it 'calculates the length of the sections' do
      expect(subject.length_of_original_file).to(eq(7))
      expect(subject.length_of_code_section).to(eq(5))
      expect(subject.length_of_csv_section).to(eq(2))
    end

    it 'sets counts to 0' do
      expect(subject.line_number).to(eq(1))
      expect(subject.row_index).to(be_nil)
      expect(subject.cell_index).to(be_nil)
    end
  end

  describe '#in_scope' do
    let(:expand) { build(:expand, repetitions: 10, starts_at: 10) }

    subject { runtime }

    context 'when var_id is undefined' do
      let(:var_id) { :foo }

      it 'raises a SyntaxError' do
        expect { runtime.in_scope?(var_id) }
          .to(raise_error(::CSVPlusPlus::Error::SyntaxError))
      end
    end

    context 'when it is not scoped to an expand' do
      let(:var_id) { :foo }
      let(:variables) { { foo: build(:cell_reference, ref: 'A1') } }

      it { is_expected.to(be_in_scope(var_id)) }
    end

    context 'when runtime#cell is outside the expand' do
      let(:var_id) { :foo }
      let(:variables) { { foo: build(:cell_reference, cell_index: 0, scoped_to_expand: expand) } }

      it { is_expected.not_to(be_in_scope(var_id)) }
    end

    context 'when runtime#cell is within the expand' do
      let(:var_id) { :foo }
      let(:row_index) { 15 }
      let(:variables) { { foo: build(:cell_reference, cell_index: 0, scoped_to_expand: expand) } }

      it { is_expected.to(be_in_scope(var_id)) }
    end
  end

  describe '#map_lines' do
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

      it 'does not increment row_index' do
        expect(runtime.map_lines(lines) { runtime.row_index }).to(eq([nil, nil, nil]))
      end
    end
  end

  describe '#map_row' do
    let(:row) { %w[cell1 cell2 cell3] }

    it 'emits each cell and index' do
      expect { |block| runtime.map_row(row, &block) }
        .to(yield_successive_args(['cell1', 0], ['cell2', 1], ['cell3', 2]))
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

    context 'with cells_too: true' do
      it 'iterates through all the cells' do
        expect { |block| runtime.map_rows(rows, cells_too: true, &block) }
          .to(
            yield_successive_args(
              ['foo', 0],
              ['bar', 1],
              ['baz', 2],
              ['foo1', 0],
              ['bar1', 1],
              ['baz1', 2],
              ['foo2', 0],
              ['bar2', 1],
              ['baz2', 2]
            )
          )
      end
    end
  end

  describe '#rownum' do
    subject { runtime.rownum }

    it { is_expected.to(eq(1)) }

    context 'when @row_index is not set' do
      let(:row_index) { nil }

      it { is_expected.to(be_nil) }
    end
  end

  describe '#builtin_variable?' do
    let(:var) { :rownum }

    subject { runtime }

    it { is_expected.to(be_builtin_variable(var)) }

    context 'with a non-runtime var' do
      let(:var) { :foo }

      it { is_expected.not_to(be_builtin_variable(var)) }
    end
  end
end

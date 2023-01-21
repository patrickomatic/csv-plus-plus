# frozen_string_literal: true

require 'cell'
require 'modifier'

describe ::CSVPlusPlus::Cell do
  let(:modifier) { build(:modifier) }
  let(:cell) { described_class.new(row_index: 0, index: 1, value:, modifier:) }
  let(:compiler) { build(:compiler) }

  describe '#to_s' do
    let(:value) { 'foo' }
    subject { cell.to_s }

    it { is_expected.to(match(/Cell\(index: 1, row_index: 0, value: foo, modifier: Modifier\(.+\)/)) }
  end

  describe '#value' do
    subject { cell.value }

    context 'with leading and trailing spaces' do
      let(:value) { '   test   ' }
      it { is_expected.to(eq('test')) }
    end

    context "when it's all spaces" do
      let(:value) { '     ' }
      it { is_expected.to(be_nil) }
    end

    context "when it's nil" do
      let(:value) { nil }
      it { is_expected.to(be_nil) }
    end
  end

  describe '#to_csv' do
    let(:ast) { nil }
    let(:cell) { build(:cell, ast:, value:) }

    subject { cell.to_csv }

    context 'with a nil value' do
      let(:value) { nil }

      it { is_expected.to(be_nil) }
    end

    context 'without an ast' do
      let(:value) { 'foo' }
      it { is_expected.to(eq(value)) }
    end

    context 'with a function' do
      let(:value) { '=MULTIPLY(5, 5)' }
      let(:ast) do
        build(:fn_call, name: 'MULTIPLY', arguments: [build(:number, n: 5), build(:number, n: 5)])
      end

      it { is_expected.to(eq(value)) }
    end

    context 'with a variable' do
      let(:value) { '=$$foo' }
      let(:ast) { build(:variable, id: 'foo') }

      it { is_expected.to(eq(value)) }
    end

    context 'with a number' do
      let(:value) { '=5' }
      let(:ast) { build(:number, n: 5) }

      it { is_expected.to(eq(value)) }
    end

    context 'with a boolean' do
      let(:value) { '=TRUE' }
      let(:ast) { build(:boolean_true) }

      it { is_expected.to(eq(value)) }
    end

    context 'with a cell reference' do
      let(:value) { '=C1' }
      let(:ast) { build(:cell_reference, ref: 'C1') }

      it { is_expected.to(eq(value)) }
    end

    context 'with a nested function' do
      let(:value) { '=MULTIPLY(5, FOO(1, 2))' }
      let(:ast) do
        build(
          :fn_call,
          name: 'MULTIPLY',
          arguments: [
            build(:number, n: 5),
            build(:fn_call, name: 'foo', arguments: [build(:number_one), build(:number_two)])
          ]
        )
      end

      it { is_expected.to(eq(value)) }
    end
  end
end

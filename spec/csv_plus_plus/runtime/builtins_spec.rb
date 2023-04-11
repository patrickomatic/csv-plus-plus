# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Runtime::Builtins do
  let(:args) { [] }
  let(:runtime) { build(:runtime, row_index: 1, cell_index: 2) }

  describe '.VARIABLES' do
    subject { described_class::VARIABLES[variable].call(runtime, args) }

    describe 'cellnum' do
      let(:variable) { :cellnum }

      it { is_expected.to(eq(build(:number, n: 3))) }
    end

    describe 'cellref' do
      let(:variable) { :cellref }

      it { is_expected.to(eq(build(:cell_reference, cell_index: 2, row_index: 1))) }
    end

    describe 'rowabove' do
      let(:variable) { :rowabove }

      it { is_expected.to(eq(build(:cell_reference, row_index: 0))) }
    end

    describe 'rowbelow' do
      let(:variable) { :rowbelow }

      it { is_expected.to(eq(build(:cell_reference, row_index: 2))) }
    end

    describe 'rownum' do
      let(:variable) { :rownum }

      it { is_expected.to(eq(build(:number, n: 2))) }
    end

    describe 'rowref' do
      let(:variable) { :rowref }

      it { is_expected.to(eq(build(:cell_reference, row_index: 1))) }
    end
  end

  describe '.FUNCTIONS' do
    subject { described_class::FUNCTIONS[function].call(runtime, args) }

    describe 'cellabove' do
      let(:args) { [build(:cell_reference, cell_index: 2)] }
      let(:function) { :cellabove }

      it { is_expected.to(eq(build(:cell_reference, ref: 'C1'))) }
    end

    describe 'celladjacent' do
      let(:args) { [build(:cell_reference, ref: 'Z')] }
      let(:function) { :celladjacent }

      it { is_expected.to(eq(build(:cell_reference, ref: 'Z2'))) }
    end

    describe 'cellbelow' do
      let(:args) { [build(:cell_reference, ref: 'B')] }
      let(:function) { :cellbelow }

      it { is_expected.to(eq(build(:cell_reference, ref: 'B3'))) }
    end
  end
end

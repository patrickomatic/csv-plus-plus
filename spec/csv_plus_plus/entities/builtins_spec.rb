# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::Builtins do
  let(:runtime) { build(:runtime, row_index: 1, cell_index: 2) }

  describe '.VARIABLES' do
    subject { described_class::VARIABLES[variable].resolve_fn.call(runtime) }

    describe 'cellnum' do
      let(:variable) { :cellnum }

      it { is_expected.to(eq(build(:number, n: 3))) }
    end

    describe 'cellref' do
      let(:variable) { :cellref }

      it { is_expected.to(eq(build(:cell_reference, ref: 'C2'))) }
    end

    describe 'rowabove' do
      let(:variable) { :rowabove }

      it { is_expected.to(eq(build(:cell_reference, ref: '1'))) }
    end

    describe 'rowbelow' do
      let(:variable) { :rowbelow }

      it { is_expected.to(eq(build(:cell_reference, ref: '3'))) }
    end

    describe 'rownum' do
      let(:variable) { :rownum }

      it { is_expected.to(eq(build(:number, n: 2))) }
    end

    describe 'rowref' do
      let(:variable) { :rowref }

      it { is_expected.to(eq(build(:cell_reference, ref: '2'))) }
    end
  end

  describe '.FUNCTIONS' do
    subject { described_class::FUNCTIONS[function].resolve_fn.call(runtime, args) }

    describe 'cellabove' do
      let(:args) { ['C'] }
      let(:function) { :cellabove }

      it { is_expected.to(eq(build(:cell_reference, ref: 'C1'))) }
    end

    describe 'celladjacent' do
      let(:args) { ['Z'] }
      let(:function) { :celladjacent }

      it { is_expected.to(eq(build(:cell_reference, ref: 'Z2'))) }
    end

    describe 'cellbelow' do
      let(:args) { ['B'] }
      let(:function) { :cellbelow }

      it { is_expected.to(eq(build(:cell_reference, ref: 'B3'))) }
    end
  end
end

# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Runtime::Runtime do
  let(:row_index) { 0 }
  let(:cell_index) { 0 }
  let(:variables) { {} }
  let(:runtime) { build(:runtime, cell_index:, row_index:, variables:) }

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

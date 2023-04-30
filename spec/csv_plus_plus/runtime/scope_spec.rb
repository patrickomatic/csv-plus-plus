# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Runtime::Scope do
  let(:functions) { {} }
  let(:variables) { {} }
  let(:scope) { build(:scope, variables:, functions:) }

  subject { scope }

  describe '#def_variable' do
    let(:id) { :foo }
    let(:value) { build(:number_one) }

    before { scope.def_variable(id, value) }

    it 'sets the values in @variables' do
      expect(scope.variables).to(eq({ foo: build(:number_one) }))
    end
  end

  describe '#def_variables' do
    let(:var_foo) { build(:variable_foo) }
    let(:id) { :foo }

    before { scope.def_variable(id, var_foo) }

    it 'sets the function in @variables' do
      expect(scope.variables).to(eq({ foo: build(:variable_foo) }))
    end

    it 'overwrites previous definitions' do
      var_bar = build(:variable_bar)
      scope.def_variable(:foo, var_bar)
      expect(scope.variables).to(eq({ foo: build(:variable_bar) }))
    end
  end

  describe '#def_function' do
    let(:fn_foo) { build(:fn_foo) }
    let(:id) { :foo }

    before { scope.def_function(id, fn_foo) }

    it 'sets the function in @functions' do
      expect(scope.functions).to(eq({ foo: build(:fn_foo) }))
    end

    it 'overwrites previous definitions' do
      fn_bar = build(:fn_bar)
      scope.def_function(:foo, fn_bar)
      expect(scope.functions).to(eq({ foo: build(:fn_bar) }))
    end
  end

  describe '#in_scope?' do
    let(:row_index) { 0 }
    let(:cell_index) { 0 }
    let(:position) { build(:position, cell_index:, row_index:) }
    let(:expand) { build(:expand, repetitions: 10, starts_at: 10) }

    context 'when var_id is undefined' do
      let(:var_id) { :foo }

      it { is_expected.not_to(be_in_scope(var_id, position)) }
    end

    context 'when it is not scoped to an expand' do
      let(:var_id) { :foo }
      let(:variables) { { foo: build(:reference, ref: 'A1') } }

      it { is_expected.to(be_in_scope(var_id, position)) }
    end

    context 'when scope#cell is outside the expand' do
      let(:var_id) { :foo }
      let(:a1_ref) { build(:a1_reference, cell_index: 0, scoped_to_expand: expand) }
      let(:variables) { { foo: build(:reference, a1_ref:) } }

      it { is_expected.not_to(be_in_scope(var_id, position)) }
    end

    context 'when scope#cell is within the expand' do
      let(:var_id) { :foo }
      let(:row_index) { 15 }
      let(:a1_ref) { build(:a1_reference, cell_index: 0, scoped_to_expand: expand) }
      let(:variables) { { foo: build(:reference, a1_ref:) } }

      it { is_expected.to(be_in_scope(var_id, position)) }
    end
  end
end

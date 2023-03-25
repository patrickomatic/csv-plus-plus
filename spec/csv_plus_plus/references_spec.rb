# frozen_string_literal: true

describe ::CSVPlusPlus::References do
  subject(:references) { described_class.new }

  describe '#initialize' do
    it 'initializes @functions' do
      expect(subject.functions).to(eq([]))
    end

    it 'initializes @variables' do
      expect(subject.variables).to(eq([]))
    end
  end

  describe '#empty?' do
    context 'without any references' do
      it { is_expected.to(be_empty) }
    end

    context 'with references' do
      before { subject.variables << build(:variable_foo) }

      it { is_expected.not_to(be_empty) }
    end
  end

  describe '.extract' do
    let(:functions) { { foo: build(:fn_foo) } }
    let(:variables) { { bar: build(:cell_reference, ref: 'A1') } }
    let(:scope) { build(:scope, functions:, variables:) }
    let(:runtime) { build(:runtime) }
    let(:ast) { build(:number_one) }

    subject { described_class.extract(ast, scope, runtime) }

    it 'finds no references' do
      expect(subject.functions).to(be_empty)
      expect(subject.variables).to(be_empty)
    end

    context 'with a variable' do
      let(:ast) { build(:variable_bar) }

      it 'finds variable references' do
        expect(subject.functions).to(be_empty)
        expect(subject.variables).to(eq([ast]))
      end
    end

    context 'with a builtin function' do
      let(:ast) { build(:fn_call, name: :celladjacent) }

      it 'finds function references' do
        expect(subject.functions).to(eq([ast]))
        expect(subject.variables).to(be_empty)
      end
    end

    context 'with a defined function' do
      let(:ast) { build(:fn_call, name: :foo) }

      it 'finds function references' do
        expect(subject.functions).to(eq([ast]))
        expect(subject.variables).to(be_empty)
      end
    end

    context 'with a native spreadsheet function' do
      let(:ast) { build(:fn_call, name: :add) }

      it 'finds no references' do
        expect(subject.functions).to(be_empty)
        expect(subject.variables).to(be_empty)
      end
    end
  end

  describe '#==' do
    let(:a) { described_class.new }
    let(:b) { described_class.new }

    subject { a == b }

    it { is_expected.to(be(true)) }

    context 'when one has references' do
      before { a.variables << build(:variable_foo) }

      it { is_expected.to(be(false)) }
    end

    context 'when both have different references' do
      before do
        a.variables << build(:variable_foo)
        b.variables << build(:variable_bar)
      end

      it { is_expected.to(be(false)) }
    end

    context 'when both have the same references' do
      before do
        a.variables << build(:variable_foo)
        b.variables << build(:variable_foo)
      end

      it { is_expected.to(be(true)) }
    end
  end
end

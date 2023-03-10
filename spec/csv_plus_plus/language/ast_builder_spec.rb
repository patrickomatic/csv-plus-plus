# frozen_string_literal: true

describe ::CSVPlusPlus::Language::ASTBuilder do
  let(:builder) { ::Class.new.extend(described_class) }

  describe '#ref' do
    let(:row_index) { nil }
    let(:cell_index) { nil }

    subject { builder.ref }

    it { is_expected.to(be_nil) }

    describe 'when row_index or cell_index is specified' do
      subject { builder.ref(row_index:, cell_index:).cell_reference }

      context 'with a row_index' do
        let(:row_index) { 0 }

        it { is_expected.to(eq('1')) }
      end

      context 'with cell_index = 1' do
        let(:cell_index) { 0 }

        it { is_expected.to(eq('A')) }
      end

      context 'with cell_index = 25' do
        let(:cell_index) { 25 }

        it { is_expected.to(eq('Z')) }
      end

      context 'with cell_index = 26' do
        let(:cell_index) { 26 }

        it { is_expected.to(eq('AA')) }
      end

      context 'with cell_index = 27' do
        let(:cell_index) { 27 }

        it { is_expected.to(eq('AB')) }
      end

      context 'with cell_index = 28' do
        let(:cell_index) { 28 }

        it { is_expected.to(eq('AC')) }
      end

      context 'with cell_index = 80' do
        let(:cell_index) { 80 }

        it { is_expected.to(eq('CC')) }
      end

      context 'with both row_index and cell_index' do
        let(:cell_index) { 5 }
        let(:row_index) { 1 }

        it { is_expected.to(eq('F2')) }
      end
    end
  end

  describe '#method_missing' do
    let(:method) { :foo }

    subject { builder.method_missing(method, false) }

    it 'raises for an unknown method' do
      expect { subject }
        .to(raise_error(::NoMethodError))
    end

    context 'with a method_name corresponding to a type' do
      let(:method) { :boolean }

      it 'instantiates instances by their type' do
        expect(subject).to(be_a(::CSVPlusPlus::Language::Entities::Boolean))
      end
    end
  end

  describe '#respond_to?' do
    let(:method) { :foo }
    subject { builder.respond_to?(method) }

    it { is_expected.to(be(false)) }

    context 'with an entity type' do
      let(:method) { :boolean }

      it { is_expected.to(be(true)) }
    end

    context 'with a method that otherwise exists' do
      let(:method) { :ref }

      it { is_expected.to(be(true)) }
    end
  end
end

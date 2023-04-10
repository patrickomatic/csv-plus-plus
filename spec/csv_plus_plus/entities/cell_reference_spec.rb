# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::CellReference do
  subject(:entity) { described_class.new(ref: 'A1') }

  describe '.valid_cell_reference?' do
    let(:cell_reference_string) { 'A1' }

    subject { described_class.valid_cell_reference?(cell_reference_string) }

    it { is_expected.to(be(true)) }

    context 'with a range' do
      let(:cell_reference_string) { 'A1:B2' }
      it { is_expected.to(be(true)) }
    end

    context 'with a sheet name' do
      let(:cell_reference_string) { 'Sheet1!A1' }

      it { is_expected.to(be(true)) }
    end

    context 'with a sheet name and range' do
      let(:cell_reference_string) { 'Sheet1!A1:B2' }

      it { is_expected.to(be(true)) }
    end

    context 'with a sheet name with quotes' do
      let(:cell_reference_string) { "'Test 1902393 =[lc2k2; Sheet'!A1:B2" }

      it { is_expected.to(be(true)) }
    end

    context 'not a cell reference' do
      let(:cell_reference_string) { '.>foo' }

      it { is_expected.not_to(be(true)) }
    end
  end

  describe '#initialize' do
    let(:cell_index) { nil }
    let(:row_index) { nil }
    let(:ref) { nil }

    subject { described_class.new(ref:, row_index:, cell_index:) }

    it 'requires cell_index, ref or row_index' do
      expect { described_class.new }
        .to(raise_error(::ArgumentError))
    end

    context 'when ref = A1' do
      let(:ref) { 'A1' }

      it 'sets @type' do
        expect(subject.type).to(eq(::CSVPlusPlus::Entities::Type::CellReference))
      end

      it 'parses into cell_index and row_index' do
        expect(subject.cell_index).to(eq(0))
        expect(subject.row_index).to(eq(0))
        expect(subject.sheet_name).to(be_nil)
      end
    end

    context 'when ref = 2' do
      let(:ref) { '2' }

      it 'parses into cell_index and row_index' do
        expect(subject.cell_index).to(be_nil)
        expect(subject.row_index).to(eq(1))
      end
    end

    context 'when ref = B' do
      let(:ref) { 'B' }

      it 'parses into cell_index and row_index' do
        expect(subject.cell_index).to(eq(1))
        expect(subject.row_index).to(be_nil)
      end
    end

    context 'when ref = CC25' do
      let(:ref) { 'CC25' }

      it 'parses into cell_index and row_index' do
        expect(subject.cell_index).to(eq(80))
        expect(subject.row_index).to(eq(24))
      end
    end

    context 'with ref = AA' do
      let(:ref) { 'AA' }

      it 'parses into cell_index' do
        expect(subject.cell_index).to(eq(26))
      end
    end

    context 'with ref = AB' do
      let(:ref) { 'AB' }

      it 'parses into cell_index' do
        expect(subject.cell_index).to(eq(27))
      end
    end

    context 'with ref = AC' do
      let(:ref) { 'AC' }

      it 'parses into cell_index' do
        expect(subject.cell_index).to(eq(28))
        expect(subject.sheet_name).to(be_nil)
      end
    end

    context 'with ref = Foo!A1' do
      let(:ref) { 'Foo!A1' }

      it 'parses the sheet_name also' do
        expect(subject.cell_index).to(eq(0))
        expect(subject.row_index).to(eq(0))
        expect(subject.sheet_name).to(eq('Foo'))
      end
    end

    context 'with ref = Foo!A1:B2' do
      let(:ref) { 'Foo!A1:B2' }

      it 'parses the sheet_name, upper_cell_index and upper_row_index also' do
        expect(subject.cell_index).to(eq(0))
        expect(subject.row_index).to(eq(0))
        expect(subject.sheet_name).to(eq('Foo'))
        expect(subject.upper_cell_index).to(eq(1))
        expect(subject.upper_row_index).to(eq(1))
      end
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:cell_reference, ref: 'A1'))) }

    it { is_expected.not_to(eq(build(:cell_reference, ref: 'Z5'))) }
    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
    it { is_expected.not_to(eq(build(:cell_reference, ref: 'A1:B2'))) }
    it { is_expected.not_to(eq(build(:cell_reference, ref: 'Foo!A1'))) }
  end

  describe '#evaluate' do
    let(:entity) { described_class.new(cell_index:, row_index:) }
    let(:cell_index) { nil }
    let(:runtime) { build(:runtime) }
    let(:row_index) { nil }

    subject { entity.evaluate(runtime) }

    context 'with a row_index' do
      let(:row_index) { 0 }

      it { is_expected.to(eq('1')) }
    end

    context 'with cell_index = 1' do
      let(:cell_index) { 0 }

      it { is_expected.to(eq('A')) }
    end

    context 'with both row_index and cell_index' do
      let(:cell_index) { 0 }
      let(:row_index) { 0 }

      it { is_expected.to(eq('A1')) }

      context 'row = 1 cell = 5' do
        let(:cell_index) { 5 }
        let(:row_index) { 1 }

        it { is_expected.to(eq('F2')) }
      end
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
  end
end
